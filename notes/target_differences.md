# WIT File Differences: WASM Guest vs Native Host

## Key Architectural Differences

### toxoid_guest/wit/world.wit (Guest Interface)
- **Purpose**: Provides a safe interface for WASM components to interact with host resources
- **Memory Model**: Works with handles/resources that abstract away direct memory access
- **Entity Handling**: Returns complete `entity` resource objects that encapsulate the functionality

```wit
// toxoid_guest/wit/world.wit
// Entity returns a full resource object that can be safely used in WASM
resource entity {
    constructor(init: entity-desc);
    from-id: static func(id: u64) -> entity;  // Returns a resource handle
    get-id: func() -> ecs-entity-t;
}

// Query returns list of entity resources
resource query {
    entities: func() -> list<entity>;  // Returns safe entity handles
}
```

### toxoid_host/wit/world.wit (Host Interface)
- **Purpose**: Provides direct interface for native code to interact with the ECS
- **Memory Model**: Works with raw pointers and memory addresses
- **Entity Handling**: Returns raw entity IDs and memory pointers

```wit
// toxoid_host/wit/world.wit
resource entity {
    from-id: static func(id: u64) -> s64;  // Returns raw pointer
    get: func(component: ecs-entity-t) -> s64;  // Returns raw pointer
}

// Query returns raw entity IDs
resource query {
    entities: func() -> list<ecs-entity-t>;  // Returns raw IDs
}
```

## Notable Implementation Differences

1. **System Description**
   - **WASM Guest**: Uses a `callback` resource type for safety
   - **Host**: Uses raw `s64` pointers for callbacks and query handles

2. **Memory Access**
   - **WASM Guest**: All memory access is mediated through resource methods
   - **Host**: Allows direct pointer manipulation with `s64` return types

3. **Component Access**
   - **WASM Guest**: Components are accessed through safe resource handles
   - **Host**: Components are accessed through raw pointers and ECS entity IDs

## Safety Considerations

The WASM guest interface (`toxoid_guest`) provides several safety guarantees:
1. No direct memory access
2. Resource-based abstraction for all operations
3. Safe entity and component handles

The host interface (`toxoid_host`) prioritizes performance and direct access:
1. Raw pointer access for native code
2. Direct ECS entity ID manipulation
3. Minimal abstraction overhead

This separation ensures that WASM components can't accidentally corrupt memory while allowing native code to maintain full performance when needed.

## Example Implementation (ToxoidEntity):
```rust
// crates/toxoid_api/src/lib.rs
impl Iter {
    pub fn entities(&self) -> Vec<Entity> {
        self.iter
            .entities()
            .iter()
            .map(|entity| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // In native mode, we get a u64 ID directly from the iterator
                    // and construct a ToxoidEntity with that ID
                    Entity {
                        entity: ToxoidEntity { id: *entity }
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // In WASM mode, we get a guest entity object that has a get_id() method
                    // We use that ID to construct a new ToxoidEntity via from_id()
                    Entity {
                        entity: ToxoidEntity::from_id(entity.get_id())
                    }
                }
            })
            .collect()
    }
}
```

1. Uses conditional compilation to handle different entity representations between native and WASM builds
2. For native mode:
   - The iterator provides raw u64 entity IDs
   - Creates a ToxoidEntity directly with that ID
3. For WASM mode:
   - The iterator provides guest entity objects through a WASM component model resource
   - Extracts the ID using get_id() method
   - Creates a new ToxoidEntity using the from_id() factory method

The key difference is that in native mode we work directly with numeric IDs, while in WASM mode we need to extract the ID from a guest entity object before creating the ToxoidEntity. Both approaches ultimately result in an Entity wrapper containing a ToxoidEntity with the correct ID.[^1].

Note [^1]: If we were not able to access the memory through ECS, we would have directly used a pointer to the entity instead, which we'd also pass back and forth as a u64. The ECS acts as a cache efficient way to access / query memory.

Let me break down the key differences between the WASM guest and native host implementations of ToxoidEntity:

### WASM Guest (Component Model Resource)
```rust
// From toxoid_guest bindings
pub struct Entity {
    handle: Resource<Entity>,  // Component model resource wrapper
}

impl GuestEntity for Entity {
    fn new(desc: EntityDesc) -> Entity { ... }
    fn get_id(&self) -> u64 { ... }
    fn add(&self, component: u64) { ... }
    // etc.
}
```
The WASM guest version:
- Is a component model resource type generated by wit-bindgen
- Uses an opaque `Resource<Entity>` handle to track the entity lifecycle
- Methods make cross-component calls to the host implementation
- Has no direct access to the underlying ECS/flecs data structures
- Must go through the component model interface for all operations

### Native Host Implementation 
```rust
// From toxoid_host
pub struct Entity { 
    pub id: ecs_entity_t  // Direct access to flecs entity ID
}

impl Entity {
    pub fn new(desc: EntityDesc) -> Entity {
        unsafe {
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            if let Some(name) = desc.name {
                ent_desc.name = c_string(&name);
            }
            let entity = ecs_entity_init(WORLD.0, &ent_desc);
            Entity { id: entity }
        }
    }
    // Direct flecs API calls
}
```

The native host version:
- Directly contains the flecs entity ID
- Makes direct FFI calls to the flecs C API
- Has full access to the ECS world and data structures
- No component model abstraction layer
- Manages memory and safety through unsafe blocks

### Key Differences

1. **Abstraction Layer**
   - Guest: Goes through component model interface
   - Host: Direct access to flecs API

2. **Memory Management**
   - Guest: Handled by component model resource system
   - Host: Manual management with unsafe FFI calls

3. **Performance**
   - Guest: Additional overhead from cross-component calls
   - Host: Direct FFI calls with minimal overhead

4. **Safety**
   - Guest: Safe abstractions provided by component model
   - Host: Unsafe blocks required for FFI

5. **Flexibility**
   - Guest: Limited to exposed component model interface
   - Host: Full access to flecs functionality

This architecture allows the WASM guest to safely interact with the ECS system while keeping the actual implementation details and unsafe code contained in the host side.



