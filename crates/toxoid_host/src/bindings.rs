#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod toxoid {
        #[allow(dead_code)]
        pub mod engine {
            #[allow(dead_code, clippy::all)]
            pub mod ecs {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type EcsEntityT = u64;
                pub type PointerT = u64;
                #[derive(Clone, Copy)]
                pub enum Relationship {
                    IsA,
                    ChildOf,
                    Custom(EcsEntityT),
                }
                impl ::core::fmt::Debug for Relationship {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Relationship::IsA => {
                                f.debug_tuple("Relationship::IsA").finish()
                            }
                            Relationship::ChildOf => {
                                f.debug_tuple("Relationship::ChildOf").finish()
                            }
                            Relationship::Custom(e) => {
                                f.debug_tuple("Relationship::Custom").field(e).finish()
                            }
                        }
                    }
                }
                #[derive(Clone, Copy)]
                pub enum Phases {
                    OnStart,
                    OnLoad,
                    PostLoad,
                    PreUpdate,
                    OnUpdate,
                    OnValidate,
                    PostUpdate,
                    PreStore,
                    OnStore,
                    Custom(EcsEntityT),
                }
                impl ::core::fmt::Debug for Phases {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Phases::OnStart => f.debug_tuple("Phases::OnStart").finish(),
                            Phases::OnLoad => f.debug_tuple("Phases::OnLoad").finish(),
                            Phases::PostLoad => {
                                f.debug_tuple("Phases::PostLoad").finish()
                            }
                            Phases::PreUpdate => {
                                f.debug_tuple("Phases::PreUpdate").finish()
                            }
                            Phases::OnUpdate => {
                                f.debug_tuple("Phases::OnUpdate").finish()
                            }
                            Phases::OnValidate => {
                                f.debug_tuple("Phases::OnValidate").finish()
                            }
                            Phases::PostUpdate => {
                                f.debug_tuple("Phases::PostUpdate").finish()
                            }
                            Phases::PreStore => {
                                f.debug_tuple("Phases::PreStore").finish()
                            }
                            Phases::OnStore => f.debug_tuple("Phases::OnStore").finish(),
                            Phases::Custom(e) => {
                                f.debug_tuple("Phases::Custom").field(e).finish()
                            }
                        }
                    }
                }
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
                    ListT,
                    U8listT,
                    U16listT,
                    U32listT,
                    U64listT,
                    I8listT,
                    I16listT,
                    I32listT,
                    I64listT,
                    F32listT,
                    F64listT,
                    PointerT,
                }
                impl ::core::fmt::Debug for MemberType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            MemberType::U8T => f.debug_tuple("MemberType::U8T").finish(),
                            MemberType::U16T => {
                                f.debug_tuple("MemberType::U16T").finish()
                            }
                            MemberType::U32T => {
                                f.debug_tuple("MemberType::U32T").finish()
                            }
                            MemberType::U64T => {
                                f.debug_tuple("MemberType::U64T").finish()
                            }
                            MemberType::I8T => f.debug_tuple("MemberType::I8T").finish(),
                            MemberType::I16T => {
                                f.debug_tuple("MemberType::I16T").finish()
                            }
                            MemberType::I32T => {
                                f.debug_tuple("MemberType::I32T").finish()
                            }
                            MemberType::I64T => {
                                f.debug_tuple("MemberType::I64T").finish()
                            }
                            MemberType::F32T => {
                                f.debug_tuple("MemberType::F32T").finish()
                            }
                            MemberType::F64T => {
                                f.debug_tuple("MemberType::F64T").finish()
                            }
                            MemberType::BoolT => {
                                f.debug_tuple("MemberType::BoolT").finish()
                            }
                            MemberType::StringT => {
                                f.debug_tuple("MemberType::StringT").finish()
                            }
                            MemberType::ListT => {
                                f.debug_tuple("MemberType::ListT").finish()
                            }
                            MemberType::U8listT => {
                                f.debug_tuple("MemberType::U8listT").finish()
                            }
                            MemberType::U16listT => {
                                f.debug_tuple("MemberType::U16listT").finish()
                            }
                            MemberType::U32listT => {
                                f.debug_tuple("MemberType::U32listT").finish()
                            }
                            MemberType::U64listT => {
                                f.debug_tuple("MemberType::U64listT").finish()
                            }
                            MemberType::I8listT => {
                                f.debug_tuple("MemberType::I8listT").finish()
                            }
                            MemberType::I16listT => {
                                f.debug_tuple("MemberType::I16listT").finish()
                            }
                            MemberType::I32listT => {
                                f.debug_tuple("MemberType::I32listT").finish()
                            }
                            MemberType::I64listT => {
                                f.debug_tuple("MemberType::I64listT").finish()
                            }
                            MemberType::F32listT => {
                                f.debug_tuple("MemberType::F32listT").finish()
                            }
                            MemberType::F64listT => {
                                f.debug_tuple("MemberType::F64listT").finish()
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
                            12 => MemberType::ListT,
                            13 => MemberType::U8listT,
                            14 => MemberType::U16listT,
                            15 => MemberType::U32listT,
                            16 => MemberType::U64listT,
                            17 => MemberType::I8listT,
                            18 => MemberType::I16listT,
                            19 => MemberType::I32listT,
                            20 => MemberType::I64listT,
                            21 => MemberType::F32listT,
                            22 => MemberType::F64listT,
                            23 => MemberType::PointerT,
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
                    pub add: Option<_rt::Vec<EcsEntityT>>,
                    pub prefab: bool,
                }
                impl ::core::fmt::Debug for EntityDesc {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("EntityDesc")
                            .field("name", &self.name)
                            .field("add", &self.add)
                            .field("prefab", &self.prefab)
                            .finish()
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
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct SortingDesc {
                    pub id: EcsEntityT,
                    pub callback: PointerT,
                }
                impl ::core::fmt::Debug for SortingDesc {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("SortingDesc")
                            .field("id", &self.id)
                            .field("callback", &self.callback)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct SystemDesc {
                    pub name: Option<_rt::String>,
                    pub tick_rate: Option<i32>,
                    pub callback: PointerT,
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
                #[derive(Clone)]
                pub struct PipelineDesc {
                    pub name: _rt::String,
                    pub query_desc: QueryDesc,
                    pub phases: _rt::Vec<EcsEntityT>,
                }
                impl ::core::fmt::Debug for PipelineDesc {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("PipelineDesc")
                            .field("name", &self.name)
                            .field("query-desc", &self.query_desc)
                            .field("phases", &self.phases)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct ObserverDesc {
                    pub name: Option<_rt::String>,
                    pub query_desc: QueryDesc,
                    pub events: _rt::Vec<Event>,
                    pub callback: PointerT,
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
                /// Convert to component-type instead and make component instances seperate
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ComponentType {
                    handle: _rt::Resource<ComponentType>,
                }
                type _ComponentTypeRep<T> = Option<T>;
                impl ComponentType {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `ComponentType`.
                    pub fn new<T: GuestComponentType>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ComponentTypeRep<T> = Some(val);
                        let ptr: *mut _ComponentTypeRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestComponentType>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestComponentType>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestComponentType>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ComponentTypeRep<T>);
                    }
                    fn as_ptr<T: GuestComponentType>(
                        &self,
                    ) -> *mut _ComponentTypeRep<T> {
                        ComponentType::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`ComponentType`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ComponentTypeBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a ComponentType>,
                }
                impl<'a> ComponentTypeBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestComponentType>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ComponentTypeRep<T> {
                        ComponentType::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for ComponentType {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
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
                type _ComponentRep<T> = Option<T>;
                impl Component {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Component`.
                    pub fn new<T: GuestComponent>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ComponentRep<T> = Some(val);
                        let ptr: *mut _ComponentRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestComponent>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestComponent>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestComponent>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ComponentRep<T>);
                    }
                    fn as_ptr<T: GuestComponent>(&self) -> *mut _ComponentRep<T> {
                        Component::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Component`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ComponentBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Component>,
                }
                impl<'a> ComponentBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestComponent>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ComponentRep<T> {
                        Component::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Component {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]component"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                /// resource tag {
                /// constructor(name: string);
                /// // get-id: func() -> ecs-entity-t;
                /// }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Entity {
                    handle: _rt::Resource<Entity>,
                }
                type _EntityRep<T> = Option<T>;
                impl Entity {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Entity`.
                    pub fn new<T: GuestEntity>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _EntityRep<T> = Some(val);
                        let ptr: *mut _EntityRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestEntity>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestEntity>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestEntity>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _EntityRep<T>);
                    }
                    fn as_ptr<T: GuestEntity>(&self) -> *mut _EntityRep<T> {
                        Entity::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Entity`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct EntityBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Entity>,
                }
                impl<'a> EntityBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestEntity>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _EntityRep<T> {
                        Entity::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Entity {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
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
                type _QueryRep<T> = Option<T>;
                impl Query {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Query`.
                    pub fn new<T: GuestQuery>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _QueryRep<T> = Some(val);
                        let ptr: *mut _QueryRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestQuery>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestQuery>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestQuery>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _QueryRep<T>);
                    }
                    fn as_ptr<T: GuestQuery>(&self) -> *mut _QueryRep<T> {
                        Query::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Query`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct QueryBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Query>,
                }
                impl<'a> QueryBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestQuery>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _QueryRep<T> {
                        Query::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Query {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
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
                pub struct Iter {
                    handle: _rt::Resource<Iter>,
                }
                type _IterRep<T> = Option<T>;
                impl Iter {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Iter`.
                    pub fn new<T: GuestIter>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _IterRep<T> = Some(val);
                        let ptr: *mut _IterRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestIter>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestIter>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestIter>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _IterRep<T>);
                    }
                    fn as_ptr<T: GuestIter>(&self) -> *mut _IterRep<T> {
                        Iter::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Iter`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct IterBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Iter>,
                }
                impl<'a> IterBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestIter>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _IterRep<T> {
                        Iter::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Iter {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]iter"]
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
                type _CallbackRep<T> = Option<T>;
                impl Callback {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Callback`.
                    pub fn new<T: GuestCallback>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _CallbackRep<T> = Some(val);
                        let ptr: *mut _CallbackRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestCallback>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestCallback>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestCallback>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _CallbackRep<T>);
                    }
                    fn as_ptr<T: GuestCallback>(&self) -> *mut _CallbackRep<T> {
                        Callback::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Callback`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct CallbackBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Callback>,
                }
                impl<'a> CallbackBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestCallback>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _CallbackRep<T> {
                        Callback::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Callback {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]callback"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct System {
                    handle: _rt::Resource<System>,
                }
                type _SystemRep<T> = Option<T>;
                impl System {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `System`.
                    pub fn new<T: GuestSystem>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _SystemRep<T> = Some(val);
                        let ptr: *mut _SystemRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestSystem>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestSystem>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestSystem>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _SystemRep<T>);
                    }
                    fn as_ptr<T: GuestSystem>(&self) -> *mut _SystemRep<T> {
                        System::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`System`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct SystemBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a System>,
                }
                impl<'a> SystemBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestSystem>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _SystemRep<T> {
                        System::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for System {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]system"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Phase {
                    handle: _rt::Resource<Phase>,
                }
                type _PhaseRep<T> = Option<T>;
                impl Phase {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Phase`.
                    pub fn new<T: GuestPhase>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _PhaseRep<T> = Some(val);
                        let ptr: *mut _PhaseRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestPhase>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestPhase>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestPhase>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _PhaseRep<T>);
                    }
                    fn as_ptr<T: GuestPhase>(&self) -> *mut _PhaseRep<T> {
                        Phase::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Phase`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct PhaseBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Phase>,
                }
                impl<'a> PhaseBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestPhase>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _PhaseRep<T> {
                        Phase::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Phase {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]phase"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Pipeline {
                    handle: _rt::Resource<Pipeline>,
                }
                type _PipelineRep<T> = Option<T>;
                impl Pipeline {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Pipeline`.
                    pub fn new<T: GuestPipeline>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _PipelineRep<T> = Some(val);
                        let ptr: *mut _PipelineRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestPipeline>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestPipeline>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestPipeline>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _PipelineRep<T>);
                    }
                    fn as_ptr<T: GuestPipeline>(&self) -> *mut _PipelineRep<T> {
                        Pipeline::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Pipeline`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct PipelineBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Pipeline>,
                }
                impl<'a> PipelineBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestPipeline>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _PipelineRep<T> {
                        Pipeline::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Pipeline {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]pipeline"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Observer {
                    handle: _rt::Resource<Observer>,
                }
                type _ObserverRep<T> = Option<T>;
                impl Observer {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Observer`.
                    pub fn new<T: GuestObserver>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ObserverRep<T> = Some(val);
                        let ptr: *mut _ObserverRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestObserver>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestObserver>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestObserver>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ObserverRep<T>);
                    }
                    fn as_ptr<T: GuestObserver>(&self) -> *mut _ObserverRep<T> {
                        Observer::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Observer`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ObserverBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Observer>,
                }
                impl<'a> ObserverBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestObserver>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ObserverRep<T> {
                        Observer::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Observer {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-drop]observer"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_component_type_cabi<
                    T: GuestComponentType,
                >(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let base4 = arg2;
                    let len4 = arg3;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 8);
                        let e4 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            _rt::string_lift(bytes3)
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
                    let len5 = arg5;
                    let result6 = ComponentType::new(
                        T::new(ComponentDesc {
                            name: _rt::string_lift(bytes0),
                            member_names: result4,
                            member_types: _rt::Vec::from_raw_parts(
                                arg4.cast(),
                                len5,
                                len5,
                            ),
                        }),
                    );
                    (result6).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_type_get_id_cabi<
                    T: GuestComponentType,
                >(arg0: *mut u8) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        ComponentTypeBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_component_cabi<T: GuestComponent>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Component::new(
                        T::new(arg0 as u64, arg1 as u64, arg2 as u64),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_component_from_ptr_cabi<T: GuestComponent>(
                    arg0: i64,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::from_ptr(arg0 as u64);
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u8,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u16,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_u64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as i8,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i8_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i8(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as i16,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i16_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i16(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_i64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: f32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_f32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f32_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f32(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: f64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_f64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f64_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> f64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f64(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_f64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_bool_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_bool(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::bool_lift(arg2 as u8),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_bool_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_bool(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_string_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    let bytes0 = _rt::Vec::from_raw_parts(arg2.cast(), len0, len0);
                    T::set_member_string(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::string_lift(bytes0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_string_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_string(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_string<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u8list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_u8list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u8list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u8list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_u8list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u16list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_u16list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u16list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u16list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_u16list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 2, 2);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_u32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_u32list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_u64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_u64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_u64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_u64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_u64list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i8list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_i8list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i8list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i8list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_i8list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i16list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_i16list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i16list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i16list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_i16list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 2, 2);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_i32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_i32list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_i64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_i64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_i64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_i64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_i64list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_f32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f32list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f32list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_f32list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_f64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: *mut u8, arg3: usize) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    T::set_member_f64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_f64list_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_f64list(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_component_get_member_f64list<
                    T: GuestComponent,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_set_member_pointer_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32, arg2: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_member_pointer(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_component_get_member_pointer_cabi<
                    T: GuestComponent,
                >(arg0: *mut u8, arg1: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_member_pointer(
                        ComponentBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_entity_cabi<T: GuestEntity>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                    arg4: *mut u8,
                    arg5: usize,
                    arg6: i32,
                    arg7: i32,
                    arg8: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result2 = Entity::new(
                        T::new(
                            EntityDesc {
                                name: match arg0 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let len0 = arg2;
                                            let bytes0 = _rt::Vec::from_raw_parts(
                                                arg1.cast(),
                                                len0,
                                                len0,
                                            );
                                            _rt::string_lift(bytes0)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                },
                                add: match arg3 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let len1 = arg5;
                                            _rt::Vec::from_raw_parts(arg4.cast(), len1, len1)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                },
                                prefab: _rt::bool_lift(arg6 as u8),
                            },
                            match arg7 {
                                0 => None,
                                1 => {
                                    let e = arg8 as u64;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                        ),
                    );
                    (result2).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_get_id_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_get_name_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_name(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_entity_get_name<T: GuestEntity>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_set_name_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    T::set_name(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_entity_from_id_cabi<T: GuestEntity>(
                    arg0: i64,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::from_id(arg0 as u64);
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_get_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_add_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add(EntityBorrow::lift(arg0 as u32 as usize).get(), arg1 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_has_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::has(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_remove_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::remove(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_add_relationship_cabi<
                    T: GuestEntity,
                >(arg0: *mut u8, arg1: i32, arg2: i64, arg3: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let v0 = match arg1 {
                        0 => Relationship::IsA,
                        1 => Relationship::ChildOf,
                        n => {
                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                            let e0 = arg2 as u64;
                            Relationship::Custom(e0)
                        }
                    };
                    T::add_relationship(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        v0,
                        arg3 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_remove_relationship_cabi<
                    T: GuestEntity,
                >(arg0: *mut u8, arg1: i32, arg2: i64, arg3: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let v0 = match arg1 {
                        0 => Relationship::IsA,
                        1 => Relationship::ChildOf,
                        n => {
                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                            let e0 = arg2 as u64;
                            Relationship::Custom(e0)
                        }
                    };
                    T::remove_relationship(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        v0,
                        arg3 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_parent_of_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::parent_of(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_child_of_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::child_of(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_parent_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::parent(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_children_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::children(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_entity_children<T: GuestEntity>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_relationships_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::relationships(
                        EntityBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_entity_relationships<T: GuestEntity>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_disable_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::disable(EntityBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_entity_enable_cabi<T: GuestEntity>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::enable(EntityBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_query_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = Query::new(
                        T::new(QueryDesc {
                            expr: _rt::string_lift(bytes0),
                        }),
                    );
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_build_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::build(QueryBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_order_by_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                    arg1: i64,
                    arg2: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::order_by(
                        QueryBorrow::lift(arg0 as u32 as usize).get(),
                        SortingDesc {
                            id: arg1 as u64,
                            callback: arg2 as u64,
                        },
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_iter_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::iter(QueryBorrow::lift(arg0 as u32 as usize).get());
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_next_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::next(QueryBorrow::lift(arg0 as u32 as usize).get());
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_count_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::count(
                        QueryBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_entities_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::entities(
                        QueryBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_query_entities<T: GuestQuery>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_query_components_cabi<T: GuestQuery>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::components(
                        QueryBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as i8,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_query_components<T: GuestQuery>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_iter_cabi<T: GuestIter>(
                    arg0: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Iter::new(T::new(arg0 as u64));
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_iter_next_cabi<T: GuestIter>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::next(IterBorrow::lift(arg0 as u32 as usize).get());
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_iter_count_cabi<T: GuestIter>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::count(IterBorrow::lift(arg0 as u32 as usize).get());
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_iter_entities_cabi<T: GuestIter>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::entities(
                        IterBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_iter_entities<T: GuestIter>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_iter_components_cabi<T: GuestIter>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::components(
                        IterBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as i8,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_iter_components<T: GuestIter>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_callback_cabi<T: GuestCallback>(
                    arg0: i64,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Callback::new(T::new(arg0 as u64));
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_callback_run_cabi<T: GuestCallback>(
                    arg0: *mut u8,
                    arg1: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::run(
                        CallbackBorrow::lift(arg0 as u32 as usize).get(),
                        Iter::from_handle(arg1 as u32),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_callback_cb_handle_cabi<T: GuestCallback>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::cb_handle(
                        CallbackBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_system_cabi<T: GuestSystem>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                    arg4: i32,
                    arg5: i64,
                    arg6: *mut u8,
                    arg7: usize,
                    arg8: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len1 = arg7;
                    let bytes1 = _rt::Vec::from_raw_parts(arg6.cast(), len1, len1);
                    let result2 = System::new(
                        T::new(SystemDesc {
                            name: match arg0 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let len0 = arg2;
                                        let bytes0 = _rt::Vec::from_raw_parts(
                                            arg1.cast(),
                                            len0,
                                            len0,
                                        );
                                        _rt::string_lift(bytes0)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            tick_rate: match arg3 {
                                0 => None,
                                1 => {
                                    let e = arg4;
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            callback: arg5 as u64,
                            query_desc: QueryDesc {
                                expr: _rt::string_lift(bytes1),
                            },
                            is_guest: _rt::bool_lift(arg8 as u8),
                        }),
                    );
                    (result2).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_get_id_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        SystemBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_build_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::build(SystemBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_order_by_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                    arg1: i64,
                    arg2: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::order_by(
                        SystemBorrow::lift(arg0 as u32 as usize).get(),
                        SortingDesc {
                            id: arg1 as u64,
                            callback: arg2 as u64,
                        },
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_callback_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::callback(
                        SystemBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_disable_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::disable(SystemBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_system_enable_cabi<T: GuestSystem>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::enable(SystemBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_phase_cabi<T: GuestPhase>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = Phase::new(T::new(_rt::string_lift(bytes0)));
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_phase_depends_on_cabi<T: GuestPhase>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let v0 = match arg1 {
                        0 => Phases::OnStart,
                        1 => Phases::OnLoad,
                        2 => Phases::PostLoad,
                        3 => Phases::PreUpdate,
                        4 => Phases::OnUpdate,
                        5 => Phases::OnValidate,
                        6 => Phases::PostUpdate,
                        7 => Phases::PreStore,
                        8 => Phases::OnStore,
                        n => {
                            debug_assert_eq!(n, 9, "invalid enum discriminant");
                            let e0 = arg2 as u64;
                            Phases::Custom(e0)
                        }
                    };
                    T::depends_on(PhaseBorrow::lift(arg0 as u32 as usize).get(), v0);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_phase_get_id_cabi<T: GuestPhase>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        PhaseBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_pipeline_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let len2 = arg5;
                    let result3 = Pipeline::new(
                        T::new(PipelineDesc {
                            name: _rt::string_lift(bytes0),
                            query_desc: QueryDesc {
                                expr: _rt::string_lift(bytes1),
                            },
                            phases: _rt::Vec::from_raw_parts(arg4.cast(), len2, len2),
                        }),
                    );
                    (result3).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_pipeline_build_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::build(PipelineBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_pipeline_add_phase_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add_phase(
                        PipelineBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_pipeline_get_id_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_id(
                        PipelineBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_pipeline_disable_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::disable(PipelineBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_pipeline_enable_cabi<T: GuestPipeline>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::enable(PipelineBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_observer_cabi<T: GuestObserver>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: i64,
                    arg8: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let base3 = arg5;
                    let len3 = arg6;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 1);
                        let e3 = {
                            let l2 = i32::from(*base.add(0).cast::<u8>());
                            Event::_lift(l2 as u8)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 1, 1);
                    let result4 = Observer::new(
                        T::new(ObserverDesc {
                            name: match arg0 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let len0 = arg2;
                                        let bytes0 = _rt::Vec::from_raw_parts(
                                            arg1.cast(),
                                            len0,
                                            len0,
                                        );
                                        _rt::string_lift(bytes0)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            query_desc: QueryDesc {
                                expr: _rt::string_lift(bytes1),
                            },
                            events: result3,
                            callback: arg7 as u64,
                            is_guest: _rt::bool_lift(arg8 as u8),
                        }),
                    );
                    (result4).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_observer_build_cabi<T: GuestObserver>(
                    arg0: *mut u8,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::build(ObserverBorrow::lift(arg0 as u32 as usize).get());
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_observer_callback_cabi<T: GuestObserver>(
                    arg0: *mut u8,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::callback(
                        ObserverBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_singleton_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add_singleton(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_singleton_cabi<T: Guest>(arg0: i64) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_singleton(arg0 as u64);
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_remove_singleton_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::remove_singleton(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_entity_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add_entity(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_remove_entity_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::remove_entity(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_has_entity_named_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::has_entity_named(_rt::string_lift(bytes0));
                    match result1 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_component_id_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::get_component_id(_rt::string_lift(bytes0));
                    _rt::as_i64(result1)
                }
                pub trait Guest {
                    type ComponentType: GuestComponentType;
                    type Component: GuestComponent;
                    type Entity: GuestEntity;
                    type Query: GuestQuery;
                    type Iter: GuestIter;
                    type Callback: GuestCallback;
                    type System: GuestSystem;
                    type Phase: GuestPhase;
                    type Pipeline: GuestPipeline;
                    type Observer: GuestObserver;
                    fn add_singleton(component_id: EcsEntityT);
                    fn get_singleton(component_id: EcsEntityT) -> u64;
                    fn remove_singleton(component_id: EcsEntityT);
                    fn add_entity(entity_id: EcsEntityT);
                    fn remove_entity(entity_id: EcsEntityT);
                    /// get-entity: func(entity-id: ecs-entity-t) -> u64;
                    /// get-entity-named: func(name: string) -> u64;
                    fn has_entity_named(name: _rt::String) -> bool;
                    fn get_component_id(component_name: _rt::String) -> EcsEntityT;
                }
                pub trait GuestComponentType: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]component-type"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]component-type"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: ComponentDesc) -> Self;
                    fn get_id(&self) -> EcsEntityT;
                }
                pub trait GuestComponent: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]component"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]component"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    /// This is a component instance so it will need a the entity it belongs to and the component type
                    /// This is required for observers / events to work
                    fn new(
                        ptr: PointerT,
                        entity: EcsEntityT,
                        component_type: EcsEntityT,
                    ) -> Self;
                    fn from_ptr(ptr: PointerT) -> PointerT;
                    fn set_member_u8(&self, offset: u32, value: u8);
                    fn get_member_u8(&self, offset: u32) -> u8;
                    fn set_member_u16(&self, offset: u32, value: u16);
                    fn get_member_u16(&self, offset: u32) -> u16;
                    fn set_member_u32(&self, offset: u32, value: u32);
                    fn get_member_u32(&self, offset: u32) -> u32;
                    fn set_member_u64(&self, offset: u32, value: u64);
                    fn get_member_u64(&self, offset: u32) -> u64;
                    fn set_member_i8(&self, offset: u32, value: i8);
                    fn get_member_i8(&self, offset: u32) -> i8;
                    fn set_member_i16(&self, offset: u32, value: i16);
                    fn get_member_i16(&self, offset: u32) -> i16;
                    fn set_member_i32(&self, offset: u32, value: i32);
                    fn get_member_i32(&self, offset: u32) -> i32;
                    fn set_member_i64(&self, offset: u32, value: i64);
                    fn get_member_i64(&self, offset: u32) -> i64;
                    fn set_member_f32(&self, offset: u32, value: f32);
                    fn get_member_f32(&self, offset: u32) -> f32;
                    fn set_member_f64(&self, offset: u32, value: f64);
                    fn get_member_f64(&self, offset: u32) -> f64;
                    fn set_member_bool(&self, offset: u32, value: bool);
                    fn get_member_bool(&self, offset: u32) -> bool;
                    fn set_member_string(&self, offset: u32, value: _rt::String);
                    fn get_member_string(&self, offset: u32) -> _rt::String;
                    fn set_member_u8list(&self, offset: u32, value: _rt::Vec<u8>);
                    fn get_member_u8list(&self, offset: u32) -> _rt::Vec<u8>;
                    fn set_member_u16list(&self, offset: u32, value: _rt::Vec<u16>);
                    fn get_member_u16list(&self, offset: u32) -> _rt::Vec<u16>;
                    fn set_member_u32list(&self, offset: u32, value: _rt::Vec<u32>);
                    fn get_member_u32list(&self, offset: u32) -> _rt::Vec<u32>;
                    fn set_member_u64list(&self, offset: u32, value: _rt::Vec<u64>);
                    fn get_member_u64list(&self, offset: u32) -> _rt::Vec<u64>;
                    fn set_member_i8list(&self, offset: u32, value: _rt::Vec<i8>);
                    fn get_member_i8list(&self, offset: u32) -> _rt::Vec<i8>;
                    fn set_member_i16list(&self, offset: u32, value: _rt::Vec<i16>);
                    fn get_member_i16list(&self, offset: u32) -> _rt::Vec<i16>;
                    fn set_member_i32list(&self, offset: u32, value: _rt::Vec<i32>);
                    fn get_member_i32list(&self, offset: u32) -> _rt::Vec<i32>;
                    fn set_member_i64list(&self, offset: u32, value: _rt::Vec<i64>);
                    fn get_member_i64list(&self, offset: u32) -> _rt::Vec<i64>;
                    fn set_member_f32list(&self, offset: u32, value: _rt::Vec<f32>);
                    fn get_member_f32list(&self, offset: u32) -> _rt::Vec<f32>;
                    fn set_member_f64list(&self, offset: u32, value: _rt::Vec<f64>);
                    fn get_member_f64list(&self, offset: u32) -> _rt::Vec<f64>;
                    fn set_member_pointer(&self, offset: u32, value: u64);
                    fn get_member_pointer(&self, offset: u32) -> u64;
                }
                pub trait GuestEntity: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]entity"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]entity"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: EntityDesc, inherits: Option<EcsEntityT>) -> Self;
                    fn get_id(&self) -> EcsEntityT;
                    fn get_name(&self) -> _rt::String;
                    fn set_name(&self, name: _rt::String);
                    fn from_id(id: u64) -> PointerT;
                    fn get(&self, component: EcsEntityT) -> PointerT;
                    fn add(&self, component: EcsEntityT);
                    fn has(&self, component: EcsEntityT) -> bool;
                    fn remove(&self, component: EcsEntityT);
                    fn add_relationship(
                        &self,
                        relationship: Relationship,
                        target: EcsEntityT,
                    );
                    fn remove_relationship(
                        &self,
                        relationship: Relationship,
                        target: EcsEntityT,
                    );
                    fn parent_of(&self, target: EcsEntityT);
                    fn child_of(&self, target: EcsEntityT);
                    fn parent(&self) -> EcsEntityT;
                    fn children(&self) -> _rt::Vec<EcsEntityT>;
                    fn relationships(&self) -> _rt::Vec<EcsEntityT>;
                    fn disable(&self);
                    fn enable(&self);
                }
                pub trait GuestQuery: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]query"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]query"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: QueryDesc) -> Self;
                    fn build(&self);
                    fn order_by(&self, desc: SortingDesc);
                    fn iter(&self) -> PointerT;
                    fn next(&self) -> bool;
                    fn count(&self) -> i32;
                    fn entities(&self) -> _rt::Vec<EcsEntityT>;
                    fn components(&self, index: i8) -> _rt::Vec<PointerT>;
                }
                pub trait GuestIter: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]iter"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]iter"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(ptr: u64) -> Self;
                    fn next(&self) -> bool;
                    fn count(&self) -> i32;
                    fn entities(&self) -> _rt::Vec<EcsEntityT>;
                    fn components(&self, index: i8) -> _rt::Vec<PointerT>;
                }
                pub trait GuestCallback: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]callback"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]callback"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(handle: u64) -> Self;
                    fn run(&self, iter: Iter);
                    /// This is prefixed because resources already have a `handle` method.
                    fn cb_handle(&self) -> PointerT;
                }
                pub trait GuestSystem: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]system"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]system"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: SystemDesc) -> Self;
                    fn get_id(&self) -> EcsEntityT;
                    fn build(&self);
                    fn order_by(&self, desc: SortingDesc);
                    fn callback(&self) -> PointerT;
                    fn disable(&self);
                    fn enable(&self);
                }
                pub trait GuestPhase: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]phase"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]phase"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(name: _rt::String) -> Self;
                    fn depends_on(&self, phase: Phases);
                    fn get_id(&self) -> EcsEntityT;
                }
                pub trait GuestPipeline: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]pipeline"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]pipeline"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: PipelineDesc) -> Self;
                    fn build(&self);
                    fn add_phase(&self, phase: EcsEntityT);
                    fn get_id(&self) -> EcsEntityT;
                    fn disable(&self);
                    fn enable(&self);
                }
                pub trait GuestObserver: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-new]observer"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]toxoid:engine/ecs")]
                            extern "C" {
                                #[link_name = "[resource-rep]observer"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(desc: ObserverDesc) -> Self;
                    fn build(&self);
                    fn callback(&self) -> PointerT;
                }
                #[doc(hidden)]
                macro_rules! __export_toxoid_engine_ecs_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "toxoid:engine/ecs#[constructor]component-type"] unsafe extern
                        "C" fn export_constructor_component_type(arg0 : * mut u8, arg1 :
                        usize, arg2 : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 :
                        usize,) -> i32 { $($path_to_types)*::
                        _export_constructor_component_type_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ComponentType > (arg0, arg1, arg2,
                        arg3, arg4, arg5) } #[export_name =
                        "toxoid:engine/ecs#[method]component-type.get-id"] unsafe extern
                        "C" fn export_method_component_type_get_id(arg0 : * mut u8,) ->
                        i64 { $($path_to_types)*::
                        _export_method_component_type_get_id_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ComponentType > (arg0) }
                        #[export_name = "toxoid:engine/ecs#[constructor]component"]
                        unsafe extern "C" fn export_constructor_component(arg0 : i64,
                        arg1 : i64, arg2 : i64,) -> i32 { $($path_to_types)*::
                        _export_constructor_component_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Component > (arg0, arg1, arg2) } #[export_name =
                        "toxoid:engine/ecs#[static]component.from-ptr"] unsafe extern "C"
                        fn export_static_component_from_ptr(arg0 : i64,) -> i64 {
                        $($path_to_types)*::
                        _export_static_component_from_ptr_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-u8"] unsafe
                        extern "C" fn export_method_component_set_member_u8(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u8"] unsafe
                        extern "C" fn export_method_component_get_member_u8(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-u16"] unsafe
                        extern "C" fn export_method_component_set_member_u16(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u16"] unsafe
                        extern "C" fn export_method_component_get_member_u16(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-u32"] unsafe
                        extern "C" fn export_method_component_set_member_u32(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_u32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u32"] unsafe
                        extern "C" fn export_method_component_get_member_u32(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_u32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-u64"] unsafe
                        extern "C" fn export_method_component_set_member_u64(arg0 : * mut
                        u8, arg1 : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_component_set_member_u64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u64"] unsafe
                        extern "C" fn export_method_component_get_member_u64(arg0 : * mut
                        u8, arg1 : i32,) -> i64 { $($path_to_types)*::
                        _export_method_component_get_member_u64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-i8"] unsafe
                        extern "C" fn export_method_component_set_member_i8(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i8"] unsafe
                        extern "C" fn export_method_component_get_member_i8(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i8_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-i16"] unsafe
                        extern "C" fn export_method_component_set_member_i16(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i16"] unsafe
                        extern "C" fn export_method_component_get_member_i16(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i16_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-i32"] unsafe
                        extern "C" fn export_method_component_set_member_i32(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_i32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i32"] unsafe
                        extern "C" fn export_method_component_get_member_i32(arg0 : * mut
                        u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_i32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-i64"] unsafe
                        extern "C" fn export_method_component_set_member_i64(arg0 : * mut
                        u8, arg1 : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_component_set_member_i64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i64"] unsafe
                        extern "C" fn export_method_component_get_member_i64(arg0 : * mut
                        u8, arg1 : i32,) -> i64 { $($path_to_types)*::
                        _export_method_component_get_member_i64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-f32"] unsafe
                        extern "C" fn export_method_component_set_member_f32(arg0 : * mut
                        u8, arg1 : i32, arg2 : f32,) { $($path_to_types)*::
                        _export_method_component_set_member_f32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-f32"] unsafe
                        extern "C" fn export_method_component_get_member_f32(arg0 : * mut
                        u8, arg1 : i32,) -> f32 { $($path_to_types)*::
                        _export_method_component_get_member_f32_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-f64"] unsafe
                        extern "C" fn export_method_component_set_member_f64(arg0 : * mut
                        u8, arg1 : i32, arg2 : f64,) { $($path_to_types)*::
                        _export_method_component_set_member_f64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-f64"] unsafe
                        extern "C" fn export_method_component_get_member_f64(arg0 : * mut
                        u8, arg1 : i32,) -> f64 { $($path_to_types)*::
                        _export_method_component_get_member_f64_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-bool"] unsafe
                        extern "C" fn export_method_component_set_member_bool(arg0 : *
                        mut u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_component_set_member_bool_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-bool"] unsafe
                        extern "C" fn export_method_component_get_member_bool(arg0 : *
                        mut u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_component_get_member_bool_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.set-member-string"] unsafe
                        extern "C" fn export_method_component_set_member_string(arg0 : *
                        mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_string_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-string"] unsafe
                        extern "C" fn export_method_component_get_member_string(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_string_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-string"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_string(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_component_get_member_string::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-u8list"] unsafe
                        extern "C" fn export_method_component_set_member_u8list(arg0 : *
                        mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_u8list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u8list"] unsafe
                        extern "C" fn export_method_component_get_member_u8list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_u8list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-u8list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_u8list(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_component_get_member_u8list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-u16list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_u16list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_u16list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u16list"] unsafe
                        extern "C" fn export_method_component_get_member_u16list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_u16list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-u16list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_u16list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_u16list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-u32list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_u32list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_u32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u32list"] unsafe
                        extern "C" fn export_method_component_get_member_u32list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_u32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-u32list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_u32list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_u32list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-u64list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_u64list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_u64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-u64list"] unsafe
                        extern "C" fn export_method_component_get_member_u64list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_u64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-u64list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_u64list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_u64list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-i8list"] unsafe
                        extern "C" fn export_method_component_set_member_i8list(arg0 : *
                        mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) {
                        $($path_to_types)*::
                        _export_method_component_set_member_i8list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i8list"] unsafe
                        extern "C" fn export_method_component_get_member_i8list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_i8list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-i8list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_i8list(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_component_get_member_i8list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-i16list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_i16list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_i16list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i16list"] unsafe
                        extern "C" fn export_method_component_get_member_i16list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_i16list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-i16list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_i16list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_i16list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-i32list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_i32list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_i32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i32list"] unsafe
                        extern "C" fn export_method_component_get_member_i32list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_i32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-i32list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_i32list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_i32list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-i64list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_i64list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_i64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-i64list"] unsafe
                        extern "C" fn export_method_component_get_member_i64list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_i64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-i64list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_i64list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_i64list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-f32list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_f32list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_f32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-f32list"] unsafe
                        extern "C" fn export_method_component_get_member_f32list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_f32list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-f32list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_f32list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_f32list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-f64list"]
                        unsafe extern "C" fn
                        export_method_component_set_member_f64list(arg0 : * mut u8, arg1
                        : i32, arg2 : * mut u8, arg3 : usize,) { $($path_to_types)*::
                        _export_method_component_set_member_f64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-f64list"] unsafe
                        extern "C" fn export_method_component_get_member_f64list(arg0 : *
                        mut u8, arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_component_get_member_f64list_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]component.get-member-f64list"]
                        unsafe extern "C" fn
                        _post_return_method_component_get_member_f64list(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_component_get_member_f64list::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0) } #[export_name
                        = "toxoid:engine/ecs#[method]component.set-member-pointer"]
                        unsafe extern "C" fn
                        export_method_component_set_member_pointer(arg0 : * mut u8, arg1
                        : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_component_set_member_pointer_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1, arg2) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]component.get-member-pointer"] unsafe
                        extern "C" fn export_method_component_get_member_pointer(arg0 : *
                        mut u8, arg1 : i32,) -> i64 { $($path_to_types)*::
                        _export_method_component_get_member_pointer_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Component > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[constructor]entity"] unsafe
                        extern "C" fn export_constructor_entity(arg0 : i32, arg1 : * mut
                        u8, arg2 : usize, arg3 : i32, arg4 : * mut u8, arg5 : usize, arg6
                        : i32, arg7 : i32, arg8 : i64,) -> i32 { $($path_to_types)*::
                        _export_constructor_entity_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Entity > (arg0, arg1, arg2, arg3, arg4, arg5, arg6,
                        arg7, arg8) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.get-id"] unsafe extern "C" fn
                        export_method_entity_get_id(arg0 : * mut u8,) -> i64 {
                        $($path_to_types)*:: _export_method_entity_get_id_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.get-name"] unsafe extern "C" fn
                        export_method_entity_get_name(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_entity_get_name_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name
                        = "cabi_post_toxoid:engine/ecs#[method]entity.get-name"] unsafe
                        extern "C" fn _post_return_method_entity_get_name(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_entity_get_name::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.set-name"] unsafe extern "C" fn
                        export_method_entity_set_name(arg0 : * mut u8, arg1 : * mut u8,
                        arg2 : usize,) { $($path_to_types)*::
                        _export_method_entity_set_name_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1, arg2) }
                        #[export_name = "toxoid:engine/ecs#[static]entity.from-id"]
                        unsafe extern "C" fn export_static_entity_from_id(arg0 : i64,) ->
                        i64 { $($path_to_types)*::
                        _export_static_entity_from_id_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.get"] unsafe extern "C" fn
                        export_method_entity_get(arg0 : * mut u8, arg1 : i64,) -> i64 {
                        $($path_to_types)*:: _export_method_entity_get_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]entity.add"] unsafe
                        extern "C" fn export_method_entity_add(arg0 : * mut u8, arg1 :
                        i64,) { $($path_to_types)*::
                        _export_method_entity_add_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Entity > (arg0, arg1) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.has"] unsafe extern "C" fn
                        export_method_entity_has(arg0 : * mut u8, arg1 : i64,) -> i32 {
                        $($path_to_types)*:: _export_method_entity_has_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]entity.remove"] unsafe
                        extern "C" fn export_method_entity_remove(arg0 : * mut u8, arg1 :
                        i64,) { $($path_to_types)*::
                        _export_method_entity_remove_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Entity > (arg0, arg1) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.add-relationship"] unsafe
                        extern "C" fn export_method_entity_add_relationship(arg0 : * mut
                        u8, arg1 : i32, arg2 : i64, arg3 : i64,) { $($path_to_types)*::
                        _export_method_entity_add_relationship_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "toxoid:engine/ecs#[method]entity.remove-relationship"] unsafe
                        extern "C" fn export_method_entity_remove_relationship(arg0 : *
                        mut u8, arg1 : i32, arg2 : i64, arg3 : i64,) {
                        $($path_to_types)*::
                        _export_method_entity_remove_relationship_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1, arg2, arg3) }
                        #[export_name = "toxoid:engine/ecs#[method]entity.parent-of"]
                        unsafe extern "C" fn export_method_entity_parent_of(arg0 : * mut
                        u8, arg1 : i64,) { $($path_to_types)*::
                        _export_method_entity_parent_of_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]entity.child-of"]
                        unsafe extern "C" fn export_method_entity_child_of(arg0 : * mut
                        u8, arg1 : i64,) { $($path_to_types)*::
                        _export_method_entity_child_of_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]entity.parent"] unsafe
                        extern "C" fn export_method_entity_parent(arg0 : * mut u8,) ->
                        i64 { $($path_to_types)*::
                        _export_method_entity_parent_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.children"] unsafe extern "C" fn
                        export_method_entity_children(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_entity_children_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name
                        = "cabi_post_toxoid:engine/ecs#[method]entity.children"] unsafe
                        extern "C" fn _post_return_method_entity_children(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_entity_children::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.relationships"] unsafe extern
                        "C" fn export_method_entity_relationships(arg0 : * mut u8,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_entity_relationships_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]entity.relationships"]
                        unsafe extern "C" fn
                        _post_return_method_entity_relationships(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_entity_relationships::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.disable"] unsafe extern "C" fn
                        export_method_entity_disable(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_entity_disable_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]entity.enable"] unsafe extern "C" fn
                        export_method_entity_enable(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_entity_enable_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Entity > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]query"] unsafe extern "C" fn
                        export_constructor_query(arg0 : * mut u8, arg1 : usize,) -> i32 {
                        $($path_to_types)*:: _export_constructor_query_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]query.build"] unsafe
                        extern "C" fn export_method_query_build(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_query_build_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]query.order-by"] unsafe extern "C" fn
                        export_method_query_order_by(arg0 : * mut u8, arg1 : i64, arg2 :
                        i64,) { $($path_to_types)*::
                        _export_method_query_order_by_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Query > (arg0, arg1, arg2) } #[export_name =
                        "toxoid:engine/ecs#[method]query.iter"] unsafe extern "C" fn
                        export_method_query_iter(arg0 : * mut u8,) -> i64 {
                        $($path_to_types)*:: _export_method_query_iter_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]query.next"] unsafe extern "C" fn
                        export_method_query_next(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_query_next_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]query.count"] unsafe extern "C" fn
                        export_method_query_count(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_query_count_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]query.entities"] unsafe extern "C" fn
                        export_method_query_entities(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_query_entities_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]query.entities"] unsafe
                        extern "C" fn _post_return_method_query_entities(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_query_entities::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]query.components"] unsafe extern "C"
                        fn export_method_query_components(arg0 : * mut u8, arg1 : i32,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_method_query_components_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]query.components"] unsafe
                        extern "C" fn _post_return_method_query_components(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_query_components::<<$ty as
                        $($path_to_types)*:: Guest >::Query > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]iter"] unsafe extern "C" fn
                        export_constructor_iter(arg0 : i64,) -> i32 {
                        $($path_to_types)*:: _export_constructor_iter_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]iter.next"] unsafe extern "C" fn
                        export_method_iter_next(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_iter_next_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]iter.count"] unsafe extern "C" fn
                        export_method_iter_count(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_iter_count_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]iter.entities"] unsafe extern "C" fn
                        export_method_iter_entities(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_iter_entities_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "cabi_post_toxoid:engine/ecs#[method]iter.entities"] unsafe
                        extern "C" fn _post_return_method_iter_entities(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_method_iter_entities::<<$ty
                        as $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]iter.components"] unsafe extern "C" fn
                        export_method_iter_components(arg0 : * mut u8, arg1 : i32,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_iter_components_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0, arg1) } #[export_name
                        = "cabi_post_toxoid:engine/ecs#[method]iter.components"] unsafe
                        extern "C" fn _post_return_method_iter_components(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_iter_components::<<$ty as
                        $($path_to_types)*:: Guest >::Iter > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]callback"] unsafe extern "C" fn
                        export_constructor_callback(arg0 : i64,) -> i32 {
                        $($path_to_types)*:: _export_constructor_callback_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Callback > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]callback.run"] unsafe extern "C" fn
                        export_method_callback_run(arg0 : * mut u8, arg1 : i32,) {
                        $($path_to_types)*:: _export_method_callback_run_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Callback > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]callback.cb-handle"]
                        unsafe extern "C" fn export_method_callback_cb_handle(arg0 : *
                        mut u8,) -> i64 { $($path_to_types)*::
                        _export_method_callback_cb_handle_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Callback > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]system"] unsafe extern "C" fn
                        export_constructor_system(arg0 : i32, arg1 : * mut u8, arg2 :
                        usize, arg3 : i32, arg4 : i32, arg5 : i64, arg6 : * mut u8, arg7
                        : usize, arg8 : i32,) -> i32 { $($path_to_types)*::
                        _export_constructor_system_cabi::<<$ty as $($path_to_types)*::
                        Guest >::System > (arg0, arg1, arg2, arg3, arg4, arg5, arg6,
                        arg7, arg8) } #[export_name =
                        "toxoid:engine/ecs#[method]system.get-id"] unsafe extern "C" fn
                        export_method_system_get_id(arg0 : * mut u8,) -> i64 {
                        $($path_to_types)*:: _export_method_system_get_id_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]system.build"] unsafe extern "C" fn
                        export_method_system_build(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_system_build_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]system.order-by"] unsafe extern "C" fn
                        export_method_system_order_by(arg0 : * mut u8, arg1 : i64, arg2 :
                        i64,) { $($path_to_types)*::
                        _export_method_system_order_by_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0, arg1, arg2) }
                        #[export_name = "toxoid:engine/ecs#[method]system.callback"]
                        unsafe extern "C" fn export_method_system_callback(arg0 : * mut
                        u8,) -> i64 { $($path_to_types)*::
                        _export_method_system_callback_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]system.disable"] unsafe extern "C" fn
                        export_method_system_disable(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_system_disable_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]system.enable"] unsafe extern "C" fn
                        export_method_system_enable(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_system_enable_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::System > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]phase"] unsafe extern "C" fn
                        export_constructor_phase(arg0 : * mut u8, arg1 : usize,) -> i32 {
                        $($path_to_types)*:: _export_constructor_phase_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Phase > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]phase.depends-on"]
                        unsafe extern "C" fn export_method_phase_depends_on(arg0 : * mut
                        u8, arg1 : i32, arg2 : i64,) { $($path_to_types)*::
                        _export_method_phase_depends_on_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Phase > (arg0, arg1, arg2) }
                        #[export_name = "toxoid:engine/ecs#[method]phase.get-id"] unsafe
                        extern "C" fn export_method_phase_get_id(arg0 : * mut u8,) -> i64
                        { $($path_to_types)*:: _export_method_phase_get_id_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Phase > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]pipeline"] unsafe extern "C" fn
                        export_constructor_pipeline(arg0 : * mut u8, arg1 : usize, arg2 :
                        * mut u8, arg3 : usize, arg4 : * mut u8, arg5 : usize,) -> i32 {
                        $($path_to_types)*:: _export_constructor_pipeline_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Pipeline > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "toxoid:engine/ecs#[method]pipeline.build"] unsafe extern "C" fn
                        export_method_pipeline_build(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_pipeline_build_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Pipeline > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]pipeline.add-phase"] unsafe extern "C"
                        fn export_method_pipeline_add_phase(arg0 : * mut u8, arg1 : i64,)
                        { $($path_to_types)*::
                        _export_method_pipeline_add_phase_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Pipeline > (arg0, arg1) }
                        #[export_name = "toxoid:engine/ecs#[method]pipeline.get-id"]
                        unsafe extern "C" fn export_method_pipeline_get_id(arg0 : * mut
                        u8,) -> i64 { $($path_to_types)*::
                        _export_method_pipeline_get_id_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Pipeline > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]pipeline.disable"] unsafe extern "C"
                        fn export_method_pipeline_disable(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_pipeline_disable_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Pipeline > (arg0) }
                        #[export_name = "toxoid:engine/ecs#[method]pipeline.enable"]
                        unsafe extern "C" fn export_method_pipeline_enable(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        _export_method_pipeline_enable_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Pipeline > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[constructor]observer"] unsafe extern "C" fn
                        export_constructor_observer(arg0 : i32, arg1 : * mut u8, arg2 :
                        usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut u8, arg6 :
                        usize, arg7 : i64, arg8 : i32,) -> i32 { $($path_to_types)*::
                        _export_constructor_observer_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Observer > (arg0, arg1, arg2, arg3, arg4, arg5, arg6,
                        arg7, arg8) } #[export_name =
                        "toxoid:engine/ecs#[method]observer.build"] unsafe extern "C" fn
                        export_method_observer_build(arg0 : * mut u8,) {
                        $($path_to_types)*:: _export_method_observer_build_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Observer > (arg0) } #[export_name =
                        "toxoid:engine/ecs#[method]observer.callback"] unsafe extern "C"
                        fn export_method_observer_callback(arg0 : * mut u8,) -> i64 {
                        $($path_to_types)*:: _export_method_observer_callback_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::Observer > (arg0) }
                        #[export_name = "toxoid:engine/ecs#add-singleton"] unsafe extern
                        "C" fn export_add_singleton(arg0 : i64,) { $($path_to_types)*::
                        _export_add_singleton_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:engine/ecs#get-singleton"] unsafe extern "C" fn
                        export_get_singleton(arg0 : i64,) -> i64 { $($path_to_types)*::
                        _export_get_singleton_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:engine/ecs#remove-singleton"] unsafe extern "C" fn
                        export_remove_singleton(arg0 : i64,) { $($path_to_types)*::
                        _export_remove_singleton_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:engine/ecs#add-entity"] unsafe extern "C" fn
                        export_add_entity(arg0 : i64,) { $($path_to_types)*::
                        _export_add_entity_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:engine/ecs#remove-entity"] unsafe extern "C" fn
                        export_remove_entity(arg0 : i64,) { $($path_to_types)*::
                        _export_remove_entity_cabi::<$ty > (arg0) } #[export_name =
                        "toxoid:engine/ecs#has-entity-named"] unsafe extern "C" fn
                        export_has_entity_named(arg0 : * mut u8, arg1 : usize,) -> i32 {
                        $($path_to_types)*:: _export_has_entity_named_cabi::<$ty > (arg0,
                        arg1) } #[export_name = "toxoid:engine/ecs#get-component-id"]
                        unsafe extern "C" fn export_get_component_id(arg0 : * mut u8,
                        arg1 : usize,) -> i64 { $($path_to_types)*::
                        _export_get_component_id_cabi::<$ty > (arg0, arg1) } const _ : ()
                        = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]component-type"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: ComponentType::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::ComponentType > (rep) } }; const _
                        : () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]component"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Component::dtor::< <$ty as $($path_to_types)*:: Guest
                        >::Component > (rep) } }; const _ : () = { #[doc(hidden)]
                        #[export_name = "toxoid:engine/ecs#[dtor]entity"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Entity::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Entity > (rep) } }; const _ : () =
                        { #[doc(hidden)] #[export_name = "toxoid:engine/ecs#[dtor]query"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Query::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Query > (rep) } }; const _ : () = {
                        #[doc(hidden)] #[export_name = "toxoid:engine/ecs#[dtor]iter"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Iter::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Iter > (rep) } }; const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]callback"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Callback::dtor::< <$ty as $($path_to_types)*:: Guest >::Callback
                        > (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]system"] #[allow(non_snake_case)] unsafe
                        extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        System::dtor::< <$ty as $($path_to_types)*:: Guest >::System >
                        (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]phase"] #[allow(non_snake_case)] unsafe
                        extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Phase::dtor::< <$ty as $($path_to_types)*:: Guest >::Phase >
                        (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]pipeline"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Pipeline::dtor::< <$ty as $($path_to_types)*:: Guest >::Pipeline
                        > (rep) } }; const _ : () = { #[doc(hidden)] #[export_name =
                        "toxoid:engine/ecs#[dtor]observer"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        Observer::dtor::< <$ty as $($path_to_types)*:: Guest >::Observer
                        > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_toxoid_engine_ecs_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
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
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        #[cfg(not(target_os = "emscripten"))]
#[cfg(not(target_os = "emscripten"))]
#[cfg(not(target_os = "emscripten"))]
wit_bindgen_rt::run_ctors_once();
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
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
macro_rules! __export_toxoid_engine_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::toxoid::engine::ecs::__export_toxoid_engine_ecs_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::toxoid::engine::ecs);
    };
}
#[doc(inline)]
pub(crate) use __export_toxoid_engine_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:toxoid:engine:toxoid-engine-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 6641] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe72\x01A\x02\x01A\x02\
\x01B\x98\x02\x01w\x04\0\x0cecs-entity-t\x03\0\0\x01w\x04\0\x09pointer-t\x03\0\x02\
\x01q\x03\x04is-a\0\0\x08child-of\0\0\x06custom\x01\x01\0\x04\0\x0crelationship\x03\
\0\x04\x01q\x0a\x08on-start\0\0\x07on-load\0\0\x09post-load\0\0\x0apre-update\0\0\
\x09on-update\0\0\x0bon-validate\0\0\x0bpost-update\0\0\x09pre-store\0\0\x08on-s\
tore\0\0\x06custom\x01\x01\0\x04\0\x06phases\x03\0\x06\x01m\x18\x04u8-t\x05u16-t\
\x05u32-t\x05u64-t\x04i8-t\x05i16-t\x05i32-t\x05i64-t\x05f32-t\x05f64-t\x06bool-\
t\x08string-t\x06list-t\x08u8list-t\x09u16list-t\x09u32list-t\x09u64list-t\x08i8\
list-t\x09i16list-t\x09i32list-t\x09i64list-t\x09f32list-t\x09f64list-t\x09point\
er-t\x04\0\x0bmember-type\x03\0\x08\x01m\x09\x06on-set\x06on-add\x09on-remove\x09\
on-delete\x10on-delete-target\x0fon-table-create\x0fon-table-delete\x0eon-table-\
empty\x0don-table-fill\x04\0\x05event\x03\0\x0a\x01ps\x01p}\x01r\x03\x04names\x0c\
member-names\x0c\x0cmember-types\x0d\x04\0\x0ecomponent-desc\x03\0\x0e\x01ks\x01\
p\x01\x01k\x11\x01r\x03\x04name\x10\x03add\x12\x06prefab\x7f\x04\0\x0bentity-des\
c\x03\0\x13\x01r\x01\x04exprs\x04\0\x0aquery-desc\x03\0\x15\x01r\x02\x02id\x01\x08\
callback\x03\x04\0\x0csorting-desc\x03\0\x17\x01kz\x01r\x05\x04name\x10\x09tick-\
rate\x19\x08callback\x03\x0aquery-desc\x16\x08is-guest\x7f\x04\0\x0bsystem-desc\x03\
\0\x1a\x01r\x03\x04names\x0aquery-desc\x16\x06phases\x11\x04\0\x0dpipeline-desc\x03\
\0\x1c\x01p\x0b\x01r\x05\x04name\x10\x0aquery-desc\x16\x06events\x1e\x08callback\
\x03\x08is-guest\x7f\x04\0\x0dobserver-desc\x03\0\x1f\x04\0\x0ecomponent-type\x03\
\x01\x04\0\x09component\x03\x01\x04\0\x06entity\x03\x01\x04\0\x05query\x03\x01\x04\
\0\x04iter\x03\x01\x04\0\x08callback\x03\x01\x04\0\x06system\x03\x01\x04\0\x05ph\
ase\x03\x01\x04\0\x08pipeline\x03\x01\x04\0\x08observer\x03\x01\x01i!\x01@\x01\x04\
desc\x0f\0+\x04\0\x1b[constructor]component-type\x01,\x01h!\x01@\x01\x04self-\0\x01\
\x04\0\x1d[method]component-type.get-id\x01.\x01i\"\x01@\x03\x03ptr\x03\x06entit\
y\x01\x0ecomponent-type\x01\0/\x04\0\x16[constructor]component\x010\x01@\x01\x03\
ptr\x03\0\x03\x04\0\x1a[static]component.from-ptr\x011\x01h\"\x01@\x03\x04self2\x06\
offsety\x05value}\x01\0\x04\0\x1f[method]component.set-member-u8\x013\x01@\x02\x04\
self2\x06offsety\0}\x04\0\x1f[method]component.get-member-u8\x014\x01@\x03\x04se\
lf2\x06offsety\x05value{\x01\0\x04\0\x20[method]component.set-member-u16\x015\x01\
@\x02\x04self2\x06offsety\0{\x04\0\x20[method]component.get-member-u16\x016\x01@\
\x03\x04self2\x06offsety\x05valuey\x01\0\x04\0\x20[method]component.set-member-u\
32\x017\x01@\x02\x04self2\x06offsety\0y\x04\0\x20[method]component.get-member-u3\
2\x018\x01@\x03\x04self2\x06offsety\x05valuew\x01\0\x04\0\x20[method]component.s\
et-member-u64\x019\x01@\x02\x04self2\x06offsety\0w\x04\0\x20[method]component.ge\
t-member-u64\x01:\x01@\x03\x04self2\x06offsety\x05value~\x01\0\x04\0\x1f[method]\
component.set-member-i8\x01;\x01@\x02\x04self2\x06offsety\0~\x04\0\x1f[method]co\
mponent.get-member-i8\x01<\x01@\x03\x04self2\x06offsety\x05value|\x01\0\x04\0\x20\
[method]component.set-member-i16\x01=\x01@\x02\x04self2\x06offsety\0|\x04\0\x20[\
method]component.get-member-i16\x01>\x01@\x03\x04self2\x06offsety\x05valuez\x01\0\
\x04\0\x20[method]component.set-member-i32\x01?\x01@\x02\x04self2\x06offsety\0z\x04\
\0\x20[method]component.get-member-i32\x01@\x01@\x03\x04self2\x06offsety\x05valu\
ex\x01\0\x04\0\x20[method]component.set-member-i64\x01A\x01@\x02\x04self2\x06off\
sety\0x\x04\0\x20[method]component.get-member-i64\x01B\x01@\x03\x04self2\x06offs\
ety\x05valuev\x01\0\x04\0\x20[method]component.set-member-f32\x01C\x01@\x02\x04s\
elf2\x06offsety\0v\x04\0\x20[method]component.get-member-f32\x01D\x01@\x03\x04se\
lf2\x06offsety\x05valueu\x01\0\x04\0\x20[method]component.set-member-f64\x01E\x01\
@\x02\x04self2\x06offsety\0u\x04\0\x20[method]component.get-member-f64\x01F\x01@\
\x03\x04self2\x06offsety\x05value\x7f\x01\0\x04\0![method]component.set-member-b\
ool\x01G\x01@\x02\x04self2\x06offsety\0\x7f\x04\0![method]component.get-member-b\
ool\x01H\x01@\x03\x04self2\x06offsety\x05values\x01\0\x04\0#[method]component.se\
t-member-string\x01I\x01@\x02\x04self2\x06offsety\0s\x04\0#[method]component.get\
-member-string\x01J\x01@\x03\x04self2\x06offsety\x05value\x0d\x01\0\x04\0#[metho\
d]component.set-member-u8list\x01K\x01@\x02\x04self2\x06offsety\0\x0d\x04\0#[met\
hod]component.get-member-u8list\x01L\x01p{\x01@\x03\x04self2\x06offsety\x05value\
\xcd\0\x01\0\x04\0$[method]component.set-member-u16list\x01N\x01@\x02\x04self2\x06\
offsety\0\xcd\0\x04\0$[method]component.get-member-u16list\x01O\x01py\x01@\x03\x04\
self2\x06offsety\x05value\xd0\0\x01\0\x04\0$[method]component.set-member-u32list\
\x01Q\x01@\x02\x04self2\x06offsety\0\xd0\0\x04\0$[method]component.get-member-u3\
2list\x01R\x01pw\x01@\x03\x04self2\x06offsety\x05value\xd3\0\x01\0\x04\0$[method\
]component.set-member-u64list\x01T\x01@\x02\x04self2\x06offsety\0\xd3\0\x04\0$[m\
ethod]component.get-member-u64list\x01U\x01p~\x01@\x03\x04self2\x06offsety\x05va\
lue\xd6\0\x01\0\x04\0#[method]component.set-member-i8list\x01W\x01@\x02\x04self2\
\x06offsety\0\xd6\0\x04\0#[method]component.get-member-i8list\x01X\x01p|\x01@\x03\
\x04self2\x06offsety\x05value\xd9\0\x01\0\x04\0$[method]component.set-member-i16\
list\x01Z\x01@\x02\x04self2\x06offsety\0\xd9\0\x04\0$[method]component.get-membe\
r-i16list\x01[\x01pz\x01@\x03\x04self2\x06offsety\x05value\xdc\0\x01\0\x04\0$[me\
thod]component.set-member-i32list\x01]\x01@\x02\x04self2\x06offsety\0\xdc\0\x04\0\
$[method]component.get-member-i32list\x01^\x01px\x01@\x03\x04self2\x06offsety\x05\
value\xdf\0\x01\0\x04\0$[method]component.set-member-i64list\x01`\x01@\x02\x04se\
lf2\x06offsety\0\xdf\0\x04\0$[method]component.get-member-i64list\x01a\x01pv\x01\
@\x03\x04self2\x06offsety\x05value\xe2\0\x01\0\x04\0$[method]component.set-membe\
r-f32list\x01c\x01@\x02\x04self2\x06offsety\0\xe2\0\x04\0$[method]component.get-\
member-f32list\x01d\x01pu\x01@\x03\x04self2\x06offsety\x05value\xe5\0\x01\0\x04\0\
$[method]component.set-member-f64list\x01f\x01@\x02\x04self2\x06offsety\0\xe5\0\x04\
\0$[method]component.get-member-f64list\x01g\x04\0$[method]component.set-member-\
pointer\x019\x04\0$[method]component.get-member-pointer\x01:\x01k\x01\x01i#\x01@\
\x02\x04desc\x14\x08inherits\xe8\0\0\xe9\0\x04\0\x13[constructor]entity\x01j\x01\
h#\x01@\x01\x04self\xeb\0\0\x01\x04\0\x15[method]entity.get-id\x01l\x01@\x01\x04\
self\xeb\0\0s\x04\0\x17[method]entity.get-name\x01m\x01@\x02\x04self\xeb\0\x04na\
mes\x01\0\x04\0\x17[method]entity.set-name\x01n\x01@\x01\x02idw\0\x03\x04\0\x16[\
static]entity.from-id\x01o\x01@\x02\x04self\xeb\0\x09component\x01\0\x03\x04\0\x12\
[method]entity.get\x01p\x01@\x02\x04self\xeb\0\x09component\x01\x01\0\x04\0\x12[\
method]entity.add\x01q\x01@\x02\x04self\xeb\0\x09component\x01\0\x7f\x04\0\x12[m\
ethod]entity.has\x01r\x04\0\x15[method]entity.remove\x01q\x01@\x03\x04self\xeb\0\
\x0crelationship\x05\x06target\x01\x01\0\x04\0\x1f[method]entity.add-relationshi\
p\x01s\x04\0\"[method]entity.remove-relationship\x01s\x01@\x02\x04self\xeb\0\x06\
target\x01\x01\0\x04\0\x18[method]entity.parent-of\x01t\x04\0\x17[method]entity.\
child-of\x01t\x04\0\x15[method]entity.parent\x01l\x01@\x01\x04self\xeb\0\0\x11\x04\
\0\x17[method]entity.children\x01u\x04\0\x1c[method]entity.relationships\x01u\x01\
@\x01\x04self\xeb\0\x01\0\x04\0\x16[method]entity.disable\x01v\x04\0\x15[method]\
entity.enable\x01v\x01i$\x01@\x01\x04desc\x16\0\xf7\0\x04\0\x12[constructor]quer\
y\x01x\x01h$\x01@\x01\x04self\xf9\0\x01\0\x04\0\x13[method]query.build\x01z\x01@\
\x02\x04self\xf9\0\x04desc\x18\x01\0\x04\0\x16[method]query.order-by\x01{\x01@\x01\
\x04self\xf9\0\0\x03\x04\0\x12[method]query.iter\x01|\x01@\x01\x04self\xf9\0\0\x7f\
\x04\0\x12[method]query.next\x01}\x01@\x01\x04self\xf9\0\0z\x04\0\x13[method]que\
ry.count\x01~\x01@\x01\x04self\xf9\0\0\x11\x04\0\x16[method]query.entities\x01\x7f\
\x01p\x03\x01@\x02\x04self\xf9\0\x05index~\0\x80\x01\x04\0\x18[method]query.comp\
onents\x01\x81\x01\x01i%\x01@\x01\x03ptrw\0\x82\x01\x04\0\x11[constructor]iter\x01\
\x83\x01\x01h%\x01@\x01\x04self\x84\x01\0\x7f\x04\0\x11[method]iter.next\x01\x85\
\x01\x01@\x01\x04self\x84\x01\0z\x04\0\x12[method]iter.count\x01\x86\x01\x01@\x01\
\x04self\x84\x01\0\x11\x04\0\x15[method]iter.entities\x01\x87\x01\x01@\x02\x04se\
lf\x84\x01\x05index~\0\x80\x01\x04\0\x17[method]iter.components\x01\x88\x01\x01i\
&\x01@\x01\x06handlew\0\x89\x01\x04\0\x15[constructor]callback\x01\x8a\x01\x01h&\
\x01@\x02\x04self\x8b\x01\x04iter\x82\x01\x01\0\x04\0\x14[method]callback.run\x01\
\x8c\x01\x01@\x01\x04self\x8b\x01\0\x03\x04\0\x1a[method]callback.cb-handle\x01\x8d\
\x01\x01i'\x01@\x01\x04desc\x1b\0\x8e\x01\x04\0\x13[constructor]system\x01\x8f\x01\
\x01h'\x01@\x01\x04self\x90\x01\0\x01\x04\0\x15[method]system.get-id\x01\x91\x01\
\x01@\x01\x04self\x90\x01\x01\0\x04\0\x14[method]system.build\x01\x92\x01\x01@\x02\
\x04self\x90\x01\x04desc\x18\x01\0\x04\0\x17[method]system.order-by\x01\x93\x01\x01\
@\x01\x04self\x90\x01\0\x03\x04\0\x17[method]system.callback\x01\x94\x01\x04\0\x16\
[method]system.disable\x01\x92\x01\x04\0\x15[method]system.enable\x01\x92\x01\x01\
i(\x01@\x01\x04names\0\x95\x01\x04\0\x12[constructor]phase\x01\x96\x01\x01h(\x01\
@\x02\x04self\x97\x01\x05phase\x07\x01\0\x04\0\x18[method]phase.depends-on\x01\x98\
\x01\x01@\x01\x04self\x97\x01\0\x01\x04\0\x14[method]phase.get-id\x01\x99\x01\x01\
i)\x01@\x01\x04desc\x1d\0\x9a\x01\x04\0\x15[constructor]pipeline\x01\x9b\x01\x01\
h)\x01@\x01\x04self\x9c\x01\x01\0\x04\0\x16[method]pipeline.build\x01\x9d\x01\x01\
@\x02\x04self\x9c\x01\x05phase\x01\x01\0\x04\0\x1a[method]pipeline.add-phase\x01\
\x9e\x01\x01@\x01\x04self\x9c\x01\0\x01\x04\0\x17[method]pipeline.get-id\x01\x9f\
\x01\x04\0\x18[method]pipeline.disable\x01\x9d\x01\x04\0\x17[method]pipeline.ena\
ble\x01\x9d\x01\x01i*\x01@\x01\x04desc\x20\0\xa0\x01\x04\0\x15[constructor]obser\
ver\x01\xa1\x01\x01h*\x01@\x01\x04self\xa2\x01\x01\0\x04\0\x16[method]observer.b\
uild\x01\xa3\x01\x01@\x01\x04self\xa2\x01\0\x03\x04\0\x19[method]observer.callba\
ck\x01\xa4\x01\x01@\x01\x0ccomponent-id\x01\x01\0\x04\0\x0dadd-singleton\x01\xa5\
\x01\x01@\x01\x0ccomponent-id\x01\0w\x04\0\x0dget-singleton\x01\xa6\x01\x04\0\x10\
remove-singleton\x01\xa5\x01\x01@\x01\x09entity-id\x01\x01\0\x04\0\x0aadd-entity\
\x01\xa7\x01\x04\0\x0dremove-entity\x01\xa7\x01\x01@\x01\x04names\0\x7f\x04\0\x10\
has-entity-named\x01\xa8\x01\x01@\x01\x0ecomponent-names\0\x01\x04\0\x10get-comp\
onent-id\x01\xa9\x01\x04\0\x11toxoid:engine/ecs\x05\0\x04\0!toxoid:engine/toxoid\
-engine-world\x04\0\x0b\x19\x01\0\x13toxoid-engine-world\x03\0\0\0G\x09producers\
\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35\
.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    #[cfg(not(target_os = "emscripten"))]
#[cfg(not(target_os = "emscripten"))]
#[cfg(not(target_os = "emscripten"))]
wit_bindgen_rt::maybe_link_cabi_realloc();
}
