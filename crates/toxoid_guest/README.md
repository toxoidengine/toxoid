# toxoid_guest
Guest side of Toxoid engine. Provides the scripting API for the toxoid engine through the WASM Component model guest API. 

The WIT file allows for use of the WASM Component model "resources" which are handles to objects in the host side. The host side only contains u64 IDs and pointers to the objects directly.