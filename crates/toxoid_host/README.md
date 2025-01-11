# toxoid_host
Native / host side of the toxoid engine / ECS API. 

The WIT file does not allow for use of the WASM Component model "resources" which are handles to objects in the host side. The host side contains more than just u64 IDs and pointers to the objects directly.