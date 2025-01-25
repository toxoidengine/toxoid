#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_init_cabi<T: Guest>() {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    T::init();
}
pub trait Guest {
    fn init();
}
#[doc(hidden)]
#[macro_export]
macro_rules! __export_world_toxoid_component_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "init"] unsafe extern "C" fn export_init() {
        $($path_to_types)*:: _export_init_cabi::<$ty > () } };
    };
}
#[doc(hidden)]
pub use __export_world_toxoid_component_world_cabi;
#[allow(dead_code)]
pub mod toxoid_component {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code, clippy::all)]
        pub mod ecs {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type EcsEntityT = u64;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum MemberType {
                U8T,
                U16T,
                U32T,
                U64T,
                I8T,
                I16T,
                I32T,
                I64T,
                F32T,
                F64T,
                BoolT,
                StringT,
                ArrayT,
                U32arrayT,
                F32arrayT,
                PointerT,
            }
            impl ::core::fmt::Debug for MemberType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        MemberType::U8T => f.debug_tuple("MemberType::U8T").finish(),
                        MemberType::U16T => f.debug_tuple("MemberType::U16T").finish(),
                        MemberType::U32T => f.debug_tuple("MemberType::U32T").finish(),
                        MemberType::U64T => f.debug_tuple("MemberType::U64T").finish(),
                        MemberType::I8T => f.debug_tuple("MemberType::I8T").finish(),
                        MemberType::I16T => f.debug_tuple("MemberType::I16T").finish(),
                        MemberType::I32T => f.debug_tuple("MemberType::I32T").finish(),
                        MemberType::I64T => f.debug_tuple("MemberType::I64T").finish(),
                        MemberType::F32T => f.debug_tuple("MemberType::F32T").finish(),
                        MemberType::F64T => f.debug_tuple("MemberType::F64T").finish(),
                        MemberType::BoolT => f.debug_tuple("MemberType::BoolT").finish(),
                        MemberType::StringT => {
                            f.debug_tuple("MemberType::StringT").finish()
                        }
                        MemberType::ArrayT => {
                            f.debug_tuple("MemberType::ArrayT").finish()
                        }
                        MemberType::U32arrayT => {
                            f.debug_tuple("MemberType::U32arrayT").finish()
                        }
                        MemberType::F32arrayT => {
                            f.debug_tuple("MemberType::F32arrayT").finish()
                        }
                        MemberType::PointerT => {
                            f.debug_tuple("MemberType::PointerT").finish()
                        }
                    }
                }
            }
            impl MemberType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> MemberType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => MemberType::U8T,
                        1 => MemberType::U16T,
                        2 => MemberType::U32T,
                        3 => MemberType::U64T,
                        4 => MemberType::I8T,
                        5 => MemberType::I16T,
                        6 => MemberType::I32T,
                        7 => MemberType::I64T,
                        8 => MemberType::F32T,
                        9 => MemberType::F64T,
                        10 => MemberType::BoolT,
                        11 => MemberType::StringT,
                        12 => MemberType::ArrayT,
                        13 => MemberType::U32arrayT,
                        14 => MemberType::F32arrayT,
                        15 => MemberType::PointerT,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Event {
                OnSet,
                OnAdd,
                OnRemove,
                OnDelete,
                OnDeleteTarget,
                OnTableCreate,
                OnTableDelete,
                OnTableEmpty,
                OnTableFill,
            }
            impl ::core::fmt::Debug for Event {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Event::OnSet => f.debug_tuple("Event::OnSet").finish(),
                        Event::OnAdd => f.debug_tuple("Event::OnAdd").finish(),
                        Event::OnRemove => f.debug_tuple("Event::OnRemove").finish(),
                        Event::OnDelete => f.debug_tuple("Event::OnDelete").finish(),
                        Event::OnDeleteTarget => {
                            f.debug_tuple("Event::OnDeleteTarget").finish()
                        }
                        Event::OnTableCreate => {
                            f.debug_tuple("Event::OnTableCreate").finish()
                        }
                        Event::OnTableDelete => {
                            f.debug_tuple("Event::OnTableDelete").finish()
                        }
                        Event::OnTableEmpty => {
                            f.debug_tuple("Event::OnTableEmpty").finish()
                        }
                        Event::OnTableFill => {
                            f.debug_tuple("Event::OnTableFill").finish()
                        }
                    }
                }
            }
            impl Event {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Event {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Event::OnSet,
                        1 => Event::OnAdd,
                        2 => Event::OnRemove,
                        3 => Event::OnDelete,
                        4 => Event::OnDeleteTarget,
                        5 => Event::OnTableCreate,
                        6 => Event::OnTableDelete,
                        7 => Event::OnTableEmpty,
                        8 => Event::OnTableFill,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[derive(Clone)]
            pub struct ComponentDesc {
                pub name: _rt::String,
                pub member_names: _rt::Vec<_rt::String>,
                pub member_types: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for ComponentDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ComponentDesc")
                        .field("name", &self.name)
                        .field("member-names", &self.member_names)
                        .field("member-types", &self.member_types)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct EntityDesc {
                pub name: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for EntityDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("EntityDesc").field("name", &self.name).finish()
                }
            }
            #[derive(Clone)]
            pub struct QueryDesc {
                pub expr: _rt::String,
            }
            impl ::core::fmt::Debug for QueryDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("QueryDesc").field("expr", &self.expr).finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ComponentType {
                handle: _rt::Resource<ComponentType>,
            }
            impl ComponentType {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ComponentType {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]component-type"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Component {
                handle: _rt::Resource<Component>,
            }
            impl Component {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Component {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]component"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Entity {
                handle: _rt::Resource<Entity>,
            }
            impl Entity {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Entity {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]entity"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Query {
                handle: _rt::Resource<Query>,
            }
            impl Query {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Query {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]query"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Callback {
                handle: _rt::Resource<Callback>,
            }
            impl Callback {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Callback {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]callback"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            pub struct SystemDesc {
                pub name: Option<_rt::String>,
                pub tick_rate: Option<i32>,
                pub callback: Callback,
                pub query_desc: QueryDesc,
                pub is_guest: bool,
            }
            impl ::core::fmt::Debug for SystemDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("SystemDesc")
                        .field("name", &self.name)
                        .field("tick-rate", &self.tick_rate)
                        .field("callback", &self.callback)
                        .field("query-desc", &self.query_desc)
                        .field("is-guest", &self.is_guest)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct System {
                handle: _rt::Resource<System>,
            }
            impl System {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for System {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]system"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            pub struct ObserverDesc {
                pub name: Option<_rt::String>,
                pub query_desc: QueryDesc,
                pub events: _rt::Vec<Event>,
                pub callback: Callback,
                pub is_guest: bool,
            }
            impl ::core::fmt::Debug for ObserverDesc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ObserverDesc")
                        .field("name", &self.name)
                        .field("query-desc", &self.query_desc)
                        .field("events", &self.events)
                        .field("callback", &self.callback)
                        .field("is-guest", &self.is_guest)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Observer {
                handle: _rt::Resource<Observer>,
            }
            impl Observer {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Observer {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]observer"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Iter {
                handle: _rt::Resource<Iter>,
            }
            impl Iter {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Iter {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[resource-drop]iter"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl ComponentType {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(init: &ComponentDesc) -> Self {
                    unsafe {
                        let ComponentDesc {
                            name: name0,
                            member_names: member_names0,
                            member_types: member_types0,
                        } = init;
                        let vec1 = name0;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec3 = member_names0;
                        let len3 = vec3.len();
                        let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec3.len() * 8,
                            4,
                        );
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 8);
                            {
                                let vec2 = e;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(4).cast::<usize>() = len2;
                                *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let vec4 = member_types0;
                        let ptr4 = vec4.as_ptr().cast::<u8>();
                        let len4 = vec4.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]component-type"]
                            fn wit_import(
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            ptr1.cast_mut(),
                            len1,
                            result3,
                            len3,
                            ptr4.cast_mut(),
                            len4,
                        );
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        ComponentType::from_handle(ret as u32)
                    }
                }
            }
            impl ComponentType {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_id(&self) -> EcsEntityT {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component-type.get-id"]
                            fn wit_import(_: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u64
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                /// This is a component instance so it will need a the entity it belongs to and the component type
                /// This is required for observers / events to work
                pub fn new(
                    ptr: i64,
                    entity: EcsEntityT,
                    component_type: EcsEntityT,
                ) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]component"]
                            fn wit_import(_: i64, _: i64, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i64, _: i64, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            _rt::as_i64(&ptr),
                            _rt::as_i64(entity),
                            _rt::as_i64(component_type),
                        );
                        Component::from_handle(ret as u32)
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                /// TODO: Change this to offset index instead of offset so that WASM guest does not have direct access to host memory.
                pub fn set_member_u8(&self, offset: u32, value: u8) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u8"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u8(&self, offset: u32) -> u8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u8"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as u8
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_u16(&self, offset: u32, value: u16) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u16"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u16(&self, offset: u32) -> u16 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u16"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as u16
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_u32(&self, offset: u32, value: u32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u32"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u32(&self, offset: u32) -> u32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u32"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as u32
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_u64(&self, offset: u32, value: u64) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u64"]
                            fn wit_import(_: i32, _: i32, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i64(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u64(&self, offset: u32) -> u64 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u64"]
                            fn wit_import(_: i32, _: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as u64
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_i8(&self, offset: u32, value: i8) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-i8"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_i8(&self, offset: u32) -> i8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-i8"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as i8
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_i16(&self, offset: u32, value: i16) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-i16"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_i16(&self, offset: u32) -> i16 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-i16"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret as i16
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_i32(&self, offset: u32, value: i32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-i32"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_i32(&self, offset: u32) -> i32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-i32"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_i64(&self, offset: u32, value: i64) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-i64"]
                            fn wit_import(_: i32, _: i32, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_i64(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_i64(&self, offset: u32) -> i64 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-i64"]
                            fn wit_import(_: i32, _: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_f32(&self, offset: u32, value: f32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-f32"]
                            fn wit_import(_: i32, _: i32, _: f32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: f32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_f32(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_f32(&self, offset: u32) -> f32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-f32"]
                            fn wit_import(_: i32, _: i32) -> f32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> f32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_f64(&self, offset: u32, value: f64) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-f64"]
                            fn wit_import(_: i32, _: i32, _: f64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: f64) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            _rt::as_f64(&value),
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_f64(&self, offset: u32) -> f64 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-f64"]
                            fn wit_import(_: i32, _: i32) -> f64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> f64 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        ret
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_bool(&self, offset: u32, value: bool) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-bool"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            match &value {
                                true => 1,
                                false => 0,
                            },
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_bool(&self, offset: u32) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-bool"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                        );
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_string(&self, offset: u32, value: &str) {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-string"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            ptr0.cast_mut(),
                            len0,
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_string(&self, offset: u32) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-string"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&offset), ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_u32list(&self, offset: u32, value: &[u32]) {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u32list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            ptr0.cast_mut(),
                            len0,
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u32list(&self, offset: u32) -> _rt::Vec<u32> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u32list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&offset), ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_u64list(&self, offset: u32, value: &[u64]) {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-u64list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            ptr0.cast_mut(),
                            len0,
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_u64list(&self, offset: u32) -> _rt::Vec<u64> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-u64list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&offset), ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_member_f32list(&self, offset: u32, value: &[f32]) {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.set-member-f32list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&offset),
                            ptr0.cast_mut(),
                            len0,
                        );
                    }
                }
            }
            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_member_f32list(&self, offset: u32) -> _rt::Vec<f32> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]component.get-member-f32list"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&offset), ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(init: &EntityDesc) -> Self {
                    unsafe {
                        let EntityDesc { name: name0 } = init;
                        let (result2_0, result2_1, result2_2) = match name0 {
                            Some(e) => {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                (1i32, ptr1.cast_mut(), len1)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]entity"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(result2_0, result2_1, result2_2);
                        Entity::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn from_id(id: u64) -> Entity {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[static]entity.from-id"]
                            fn wit_import(_: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(_rt::as_i64(&id));
                        Entity::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_id(&self) -> EcsEntityT {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.get-id"]
                            fn wit_import(_: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u64
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_name(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.get-name"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self, component: EcsEntityT) -> Component {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.get"]
                            fn wit_import(_: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(component),
                        );
                        Component::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn add(&self, component: EcsEntityT) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.add"]
                            fn wit_import(_: i32, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(component));
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remove(&self, component: EcsEntityT) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.remove"]
                            fn wit_import(_: i32, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(component));
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn add_relationship(
                    &self,
                    relationship: EcsEntityT,
                    target: EcsEntityT,
                ) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.add-relationship"]
                            fn wit_import(_: i32, _: i64, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(relationship),
                            _rt::as_i64(target),
                        );
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remove_relationship(
                    &self,
                    relationship: EcsEntityT,
                    target: EcsEntityT,
                ) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]entity.remove-relationship"]
                            fn wit_import(_: i32, _: i64, _: i64);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(relationship),
                            _rt::as_i64(target),
                        );
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(desc: &QueryDesc) -> Self {
                    unsafe {
                        let QueryDesc { expr: expr0 } = desc;
                        let vec1 = expr0;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]query"]
                            fn wit_import(_: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(ptr1.cast_mut(), len1);
                        Query::from_handle(ret as u32)
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn expr(&self, expr: &str) {
                    unsafe {
                        let vec0 = expr;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.expr"]
                            fn wit_import(_: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0);
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn build(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.build"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn iter(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.iter"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn next(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.next"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn count(&self) -> i32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.count"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn entities(&self) -> _rt::Vec<Entity> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]query.entities"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base4 = l1;
                        let len4 = l2;
                        let mut result4 = _rt::Vec::with_capacity(len4);
                        for i in 0..len4 {
                            let base = base4.add(i * 4);
                            let e4 = {
                                let l3 = *base.add(0).cast::<i32>();
                                Entity::from_handle(l3 as u32)
                            };
                            result4.push(e4);
                        }
                        _rt::cabi_dealloc(base4, len4 * 4, 4);
                        result4
                    }
                }
            }
            impl Callback {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(handle: i64) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]callback"]
                            fn wit_import(_: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(_rt::as_i64(&handle));
                        Callback::from_handle(ret as u32)
                    }
                }
            }
            impl Callback {
                #[allow(unused_unsafe, clippy::all)]
                pub fn run(&self, iter: Iter) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]callback.run"]
                            fn wit_import(_: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, (&iter).take_handle() as i32);
                    }
                }
            }
            impl Callback {
                #[allow(unused_unsafe, clippy::all)]
                /// This is prefixed because resources already have a `handle` method.
                pub fn cb_handle(&self) -> i64 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]callback.cb-handle"]
                            fn wit_import(_: i32) -> i64;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i64 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret
                    }
                }
            }
            impl System {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(desc: SystemDesc) -> Self {
                    unsafe {
                        let SystemDesc {
                            name: name0,
                            tick_rate: tick_rate0,
                            callback: callback0,
                            query_desc: query_desc0,
                            is_guest: is_guest0,
                        } = &desc;
                        let (result2_0, result2_1, result2_2) = match name0 {
                            Some(e) => {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                (1i32, ptr1.cast_mut(), len1)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        let (result3_0, result3_1) = match tick_rate0 {
                            Some(e) => (1i32, _rt::as_i32(e)),
                            None => (0i32, 0i32),
                        };
                        let QueryDesc { expr: expr4 } = query_desc0;
                        let vec5 = expr4;
                        let ptr5 = vec5.as_ptr().cast::<u8>();
                        let len5 = vec5.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]system"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            result2_0,
                            result2_1,
                            result2_2,
                            result3_0,
                            result3_1,
                            (callback0).take_handle() as i32,
                            ptr5.cast_mut(),
                            len5,
                            match is_guest0 {
                                true => 1,
                                false => 0,
                            },
                        );
                        System::from_handle(ret as u32)
                    }
                }
            }
            impl System {
                #[allow(unused_unsafe, clippy::all)]
                pub fn build(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]system.build"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl System {
                #[allow(unused_unsafe, clippy::all)]
                pub fn callback(&self) -> Callback {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]system.callback"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Callback::from_handle(ret as u32)
                    }
                }
            }
            impl Observer {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(desc: ObserverDesc) -> Self {
                    unsafe {
                        let ObserverDesc {
                            name: name0,
                            query_desc: query_desc0,
                            events: events0,
                            callback: callback0,
                            is_guest: is_guest0,
                        } = &desc;
                        let (result2_0, result2_1, result2_2) = match name0 {
                            Some(e) => {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                (1i32, ptr1.cast_mut(), len1)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        let QueryDesc { expr: expr3 } = query_desc0;
                        let vec4 = expr3;
                        let ptr4 = vec4.as_ptr().cast::<u8>();
                        let len4 = vec4.len();
                        let vec5 = events0;
                        let len5 = vec5.len();
                        let layout5 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec5.len() * 1,
                            1,
                        );
                        let result5 = if layout5.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout5);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec5.into_iter().enumerate() {
                            let base = result5.add(i * 1);
                            {
                                *base.add(0).cast::<u8>() = (e.clone() as i32) as u8;
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]observer"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            result2_0,
                            result2_1,
                            result2_2,
                            ptr4.cast_mut(),
                            len4,
                            result5,
                            len5,
                            (callback0).take_handle() as i32,
                            match is_guest0 {
                                true => 1,
                                false => 0,
                            },
                        );
                        if layout5.size() != 0 {
                            _rt::alloc::dealloc(result5.cast(), layout5);
                        }
                        Observer::from_handle(ret as u32)
                    }
                }
            }
            impl Observer {
                #[allow(unused_unsafe, clippy::all)]
                pub fn build(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]observer.build"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl Observer {
                #[allow(unused_unsafe, clippy::all)]
                pub fn callback(&self) -> Callback {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]observer.callback"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Callback::from_handle(ret as u32)
                    }
                }
            }
            impl Iter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(ptr: i64) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[constructor]iter"]
                            fn wit_import(_: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(_rt::as_i64(&ptr));
                        Iter::from_handle(ret as u32)
                    }
                }
            }
            impl Iter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn next(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]iter.next"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Iter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn count(&self) -> i32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]iter.count"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret
                    }
                }
            }
            impl Iter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn entities(&self) -> _rt::Vec<Entity> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "toxoid-component:component/ecs")]
                        extern "C" {
                            #[link_name = "[method]iter.entities"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base4 = l1;
                        let len4 = l2;
                        let mut result4 = _rt::Vec::with_capacity(len4);
                        for i in 0..len4 {
                            let base = base4.add(i * 4);
                            let e4 = {
                                let l3 = *base.add(0).cast::<i32>();
                                Entity::from_handle(l3 as u32)
                            };
                            result4.push(e4);
                        }
                        _rt::cabi_dealloc(base4, len4 * 4, 4);
                        result4
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn add_singleton(component: EcsEntityT) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "add-singleton"]
                        fn wit_import(_: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(component));
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_singleton(component: EcsEntityT) -> Component {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "get-singleton"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(component));
                    Component::from_handle(ret as u32)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn remove_singleton(component: EcsEntityT) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "remove-singleton"]
                        fn wit_import(_: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(component));
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn add_entity(entity: EcsEntityT) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "add-entity"]
                        fn wit_import(_: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(entity));
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// get-entity: func(entity: ecs-entity-t) -> entity;
            pub fn remove_entity(entity: EcsEntityT) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "remove-entity"]
                        fn wit_import(_: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(entity));
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// get-entity-named: func(name: string) -> entity;
            pub fn has_entity_named(name: &str) -> bool {
                unsafe {
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "toxoid-component:component/ecs")]
                    extern "C" {
                        #[link_name = "has-entity-named"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(ptr0.cast_mut(), len0);
                    _rt::bool_lift(ret as u8)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod toxoid_component {
        #[allow(dead_code)]
        pub mod component {
            #[allow(dead_code, clippy::all)]
            pub mod callbacks {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Iter = super::super::super::super::toxoid_component::component::ecs::Iter;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>(arg0: i32, arg1: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::run(
                        super::super::super::super::toxoid_component::component::ecs::Iter::from_handle(
                            arg0 as u32,
                        ),
                        arg1,
                    );
                }
                pub trait Guest {
                    fn run(iter: Iter, handle: i64);
                }
                #[doc(hidden)]
                #[macro_export]
macro_rules! __export_toxoid_component_component_callbacks_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "toxoid-component:component/callbacks#run"] unsafe extern "C" fn
                        export_run(arg0 : i32, arg1 : i64,) { $($path_to_types)*::
                        _export_run_cabi::<$ty > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub use __export_toxoid_component_component_callbacks_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::alloc;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub fn as_f64<T: AsF64>(t: T) -> f64 {
        t.as_f64()
    }
    pub trait AsF64 {
        fn as_f64(self) -> f64;
    }
    impl<'a, T: Copy + AsF64> AsF64 for &'a T {
        fn as_f64(self) -> f64 {
            (*self).as_f64()
        }
    }
    impl AsF64 for f64 {
        #[inline]
        fn as_f64(self) -> f64 {
            self as f64
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
#[macro_export]
macro_rules! __export_toxoid_component_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_toxoid_component_world_cabi!($ty
        with_types_in $($path_to_types_root)*); $($path_to_types_root)*::
        exports::toxoid_component::component::callbacks::__export_toxoid_component_component_callbacks_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::toxoid_component::component::callbacks);
    };
}
#[doc(inline)]
pub use __export_toxoid_component_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:toxoid-component:component:toxoid-component-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 4309] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc8\x20\x01A\x02\x01\
A\x07\x01B\xb4\x01\x01w\x04\0\x0cecs-entity-t\x03\0\0\x01m\x10\x04u8-t\x05u16-t\x05\
u32-t\x05u64-t\x04i8-t\x05i16-t\x05i32-t\x05i64-t\x05f32-t\x05f64-t\x06bool-t\x08\
string-t\x07array-t\x0au32array-t\x0af32array-t\x09pointer-t\x04\0\x0bmember-typ\
e\x03\0\x02\x01m\x09\x06on-set\x06on-add\x09on-remove\x09on-delete\x10on-delete-\
target\x0fon-table-create\x0fon-table-delete\x0eon-table-empty\x0don-table-fill\x04\
\0\x05event\x03\0\x04\x01ps\x01p}\x01r\x03\x04names\x0cmember-names\x06\x0cmembe\
r-types\x07\x04\0\x0ecomponent-desc\x03\0\x08\x01ks\x01r\x01\x04name\x0a\x04\0\x0b\
entity-desc\x03\0\x0b\x01r\x01\x04exprs\x04\0\x0aquery-desc\x03\0\x0d\x04\0\x0ec\
omponent-type\x03\x01\x04\0\x09component\x03\x01\x04\0\x06entity\x03\x01\x04\0\x05\
query\x03\x01\x04\0\x08callback\x03\x01\x01kz\x01i\x13\x01r\x05\x04name\x0a\x09t\
ick-rate\x14\x08callback\x15\x0aquery-desc\x0e\x08is-guest\x7f\x04\0\x0bsystem-d\
esc\x03\0\x16\x04\0\x06system\x03\x01\x01p\x05\x01r\x05\x04name\x0a\x0aquery-des\
c\x0e\x06events\x19\x08callback\x15\x08is-guest\x7f\x04\0\x0dobserver-desc\x03\0\
\x1a\x04\0\x08observer\x03\x01\x04\0\x04iter\x03\x01\x01i\x0f\x01@\x01\x04init\x09\
\0\x1e\x04\0\x1b[constructor]component-type\x01\x1f\x01h\x0f\x01@\x01\x04self\x20\
\0\x01\x04\0\x1d[method]component-type.get-id\x01!\x01i\x10\x01@\x03\x03ptrx\x06\
entity\x01\x0ecomponent-type\x01\0\"\x04\0\x16[constructor]component\x01#\x01h\x10\
\x01@\x03\x04self$\x06offsety\x05value}\x01\0\x04\0\x1f[method]component.set-mem\
ber-u8\x01%\x01@\x02\x04self$\x06offsety\0}\x04\0\x1f[method]component.get-membe\
r-u8\x01&\x01@\x03\x04self$\x06offsety\x05value{\x01\0\x04\0\x20[method]componen\
t.set-member-u16\x01'\x01@\x02\x04self$\x06offsety\0{\x04\0\x20[method]component\
.get-member-u16\x01(\x01@\x03\x04self$\x06offsety\x05valuey\x01\0\x04\0\x20[meth\
od]component.set-member-u32\x01)\x01@\x02\x04self$\x06offsety\0y\x04\0\x20[metho\
d]component.get-member-u32\x01*\x01@\x03\x04self$\x06offsety\x05valuew\x01\0\x04\
\0\x20[method]component.set-member-u64\x01+\x01@\x02\x04self$\x06offsety\0w\x04\0\
\x20[method]component.get-member-u64\x01,\x01@\x03\x04self$\x06offsety\x05value~\
\x01\0\x04\0\x1f[method]component.set-member-i8\x01-\x01@\x02\x04self$\x06offset\
y\0~\x04\0\x1f[method]component.get-member-i8\x01.\x01@\x03\x04self$\x06offsety\x05\
value|\x01\0\x04\0\x20[method]component.set-member-i16\x01/\x01@\x02\x04self$\x06\
offsety\0|\x04\0\x20[method]component.get-member-i16\x010\x01@\x03\x04self$\x06o\
ffsety\x05valuez\x01\0\x04\0\x20[method]component.set-member-i32\x011\x01@\x02\x04\
self$\x06offsety\0z\x04\0\x20[method]component.get-member-i32\x012\x01@\x03\x04s\
elf$\x06offsety\x05valuex\x01\0\x04\0\x20[method]component.set-member-i64\x013\x01\
@\x02\x04self$\x06offsety\0x\x04\0\x20[method]component.get-member-i64\x014\x01@\
\x03\x04self$\x06offsety\x05valuev\x01\0\x04\0\x20[method]component.set-member-f\
32\x015\x01@\x02\x04self$\x06offsety\0v\x04\0\x20[method]component.get-member-f3\
2\x016\x01@\x03\x04self$\x06offsety\x05valueu\x01\0\x04\0\x20[method]component.s\
et-member-f64\x017\x01@\x02\x04self$\x06offsety\0u\x04\0\x20[method]component.ge\
t-member-f64\x018\x01@\x03\x04self$\x06offsety\x05value\x7f\x01\0\x04\0![method]\
component.set-member-bool\x019\x01@\x02\x04self$\x06offsety\0\x7f\x04\0![method]\
component.get-member-bool\x01:\x01@\x03\x04self$\x06offsety\x05values\x01\0\x04\0\
#[method]component.set-member-string\x01;\x01@\x02\x04self$\x06offsety\0s\x04\0#\
[method]component.get-member-string\x01<\x01py\x01@\x03\x04self$\x06offsety\x05v\
alue=\x01\0\x04\0$[method]component.set-member-u32list\x01>\x01@\x02\x04self$\x06\
offsety\0=\x04\0$[method]component.get-member-u32list\x01?\x01pw\x01@\x03\x04sel\
f$\x06offsety\x05value\xc0\0\x01\0\x04\0$[method]component.set-member-u64list\x01\
A\x01@\x02\x04self$\x06offsety\0\xc0\0\x04\0$[method]component.get-member-u64lis\
t\x01B\x01pv\x01@\x03\x04self$\x06offsety\x05value\xc3\0\x01\0\x04\0$[method]com\
ponent.set-member-f32list\x01D\x01@\x02\x04self$\x06offsety\0\xc3\0\x04\0$[metho\
d]component.get-member-f32list\x01E\x01i\x11\x01@\x01\x04init\x0c\0\xc6\0\x04\0\x13\
[constructor]entity\x01G\x01@\x01\x02idw\0\xc6\0\x04\0\x16[static]entity.from-id\
\x01H\x01h\x11\x01@\x01\x04self\xc9\0\0\x01\x04\0\x15[method]entity.get-id\x01J\x01\
@\x01\x04self\xc9\0\0s\x04\0\x17[method]entity.get-name\x01K\x01@\x02\x04self\xc9\
\0\x09component\x01\0\"\x04\0\x12[method]entity.get\x01L\x01@\x02\x04self\xc9\0\x09\
component\x01\x01\0\x04\0\x12[method]entity.add\x01M\x04\0\x15[method]entity.rem\
ove\x01M\x01@\x03\x04self\xc9\0\x0crelationship\x01\x06target\x01\x01\0\x04\0\x1f\
[method]entity.add-relationship\x01N\x04\0\"[method]entity.remove-relationship\x01\
N\x01i\x12\x01@\x01\x04desc\x0e\0\xcf\0\x04\0\x12[constructor]query\x01P\x01h\x12\
\x01@\x02\x04self\xd1\0\x04exprs\x01\0\x04\0\x12[method]query.expr\x01R\x01@\x01\
\x04self\xd1\0\x01\0\x04\0\x13[method]query.build\x01S\x04\0\x12[method]query.it\
er\x01S\x01@\x01\x04self\xd1\0\0\x7f\x04\0\x12[method]query.next\x01T\x01@\x01\x04\
self\xd1\0\0z\x04\0\x13[method]query.count\x01U\x01p\xc6\0\x01@\x01\x04self\xd1\0\
\0\xd6\0\x04\0\x16[method]query.entities\x01W\x01@\x01\x06handlex\0\x15\x04\0\x15\
[constructor]callback\x01X\x01h\x13\x01i\x1d\x01@\x02\x04self\xd9\0\x04iter\xda\0\
\x01\0\x04\0\x14[method]callback.run\x01[\x01@\x01\x04self\xd9\0\0x\x04\0\x1a[me\
thod]callback.cb-handle\x01\\\x01i\x18\x01@\x01\x04desc\x17\0\xdd\0\x04\0\x13[co\
nstructor]system\x01^\x01h\x18\x01@\x01\x04self\xdf\0\x01\0\x04\0\x14[method]sys\
tem.build\x01`\x01@\x01\x04self\xdf\0\0\x15\x04\0\x17[method]system.callback\x01\
a\x01i\x1c\x01@\x01\x04desc\x1b\0\xe2\0\x04\0\x15[constructor]observer\x01c\x01h\
\x1c\x01@\x01\x04self\xe4\0\x01\0\x04\0\x16[method]observer.build\x01e\x01@\x01\x04\
self\xe4\0\0\x15\x04\0\x19[method]observer.callback\x01f\x01@\x01\x03ptrx\0\xda\0\
\x04\0\x11[constructor]iter\x01g\x01h\x1d\x01@\x01\x04self\xe8\0\0\x7f\x04\0\x11\
[method]iter.next\x01i\x01@\x01\x04self\xe8\0\0z\x04\0\x12[method]iter.count\x01\
j\x01@\x01\x04self\xe8\0\0\xd6\0\x04\0\x15[method]iter.entities\x01k\x01@\x01\x09\
component\x01\x01\0\x04\0\x0dadd-singleton\x01l\x01@\x01\x09component\x01\0\"\x04\
\0\x0dget-singleton\x01m\x04\0\x10remove-singleton\x01l\x01@\x01\x06entity\x01\x01\
\0\x04\0\x0aadd-entity\x01n\x04\0\x0dremove-entity\x01n\x01@\x01\x04names\0\x7f\x04\
\0\x10has-entity-named\x01o\x03\0\x1etoxoid-component:component/ecs\x05\0\x01@\0\
\x01\0\x04\0\x04init\x01\x01\x02\x03\0\0\x04iter\x01B\x05\x02\x03\x02\x01\x02\x04\
\0\x04iter\x03\0\0\x01i\x01\x01@\x02\x04iter\x02\x06handlex\x01\0\x04\0\x03run\x01\
\x03\x04\0$toxoid-component:component/callbacks\x05\x03\x04\01toxoid-component:c\
omponent/toxoid-component-world\x04\0\x0b\x1c\x01\0\x16toxoid-component-world\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-\
bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
