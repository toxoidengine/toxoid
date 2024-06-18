warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_wasm_test\Cargo.toml
workspace: C:\Users\troye\dev\toxoid\toxoid\Cargo.toml
warning: C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_wasm\Cargo.toml: Found `feature = ...` in `target.'cfg(...)'.dependencies`. This key is not supported for selecting dependencies and will not work as expected. Use the [features] section instead: https://doc.rust-lang.org/cargo/reference/features.html
warning: C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_wasm\Cargo.toml: Found `feature = ...` in `target.'cfg(...)'.dependencies`. This key is not supported for selecting dependencies and will not work as expected. Use the [features] section instead: https://doc.rust-lang.org/cargo/reference/features.html
warning: unused import: `Result`
   --> crates\toxoid_api_macro\src\lib.rs:693:18
    |
693 | use syn::{Token, Result};
    |                  ^^^^^^
    |
    = note: `#[warn(unused_imports)]` on by default
    Checking toxoid_api v0.1.0 (C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_api)
warning: unused import: `crate::ecs::*`
 --> crates\toxoid_api\src\bindings.rs:3:5
  |
3 | use crate::ecs::*;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: unused import: `core::ffi::c_void`
 --> crates\toxoid_api\src\bindings.rs:4:5
  |
4 | use core::ffi::c_void;
  |     ^^^^^^^^^^^^^^^^^
warning: unused import: `crate::bindings::*`
 --> crates\toxoid_api\src\log.rs:1:5
  |
1 | use crate::bindings::*;
  |     ^^^^^^^^^^^^^^^^^^
warning: unused import: `crate::*`
 --> crates\toxoid_api\src\events.rs:1:5
  |
1 | use crate::*;
  |     ^^^^^^^^
error[E0425]: cannot find function `toxoid_make_c_string` in this scope
  --> crates\toxoid_api\src\ecs.rs:84:27
   |
84 |             ptr: unsafe { toxoid_make_c_string(rust_str) }
   |                           ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_create` in this scope
   --> crates\toxoid_api\src\ecs.rs:307:35
    |
307 |             filter_desc: unsafe { toxoid_filter_create() },
    |                                   ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:323:35
    |
323 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_with` in this scope
   --> crates\toxoid_api\src\ecs.rs:327:38
    |
327 |         self.filter_index = unsafe { toxoid_filter_with(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:340:35
    |
340 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_without` in this scope
   --> crates\toxoid_api\src\ecs.rs:345:38
    |
345 |         self.filter_index = unsafe { toxoid_filter_without(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:358:35
    |
358 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_with_or` in this scope
   --> crates\toxoid_api\src\ecs.rs:363:38
    |
363 |         self.filter_index = unsafe { toxoid_filter_with_or(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_iter` in this scope
   --> crates\toxoid_api\src\ecs.rs:368:18
    |
368 |         unsafe { toxoid_filter_iter(self.filter) }
    |                  ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_build` in this scope
   --> crates\toxoid_api\src\ecs.rs:372:32
    |
372 |         self.filter = unsafe { toxoid_filter_build(self.filter_desc) };
    |                                ^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_count` in this scope
   --> crates\toxoid_api\src\ecs.rs:398:18
    |
398 |         unsafe { toxoid_iter_count(self.iter) as usize }
    |                  ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_next` in this scope
   --> crates\toxoid_api\src\ecs.rs:402:18
    |
402 |         unsafe { toxoid_query_next(self.iter) }
    |                  ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_next` in this scope
   --> crates\toxoid_api\src\ecs.rs:406:18
    |
406 |         unsafe { toxoid_filter_next(self.iter) }
    |                  ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_entity_list` in this scope
   --> crates\toxoid_api\src\ecs.rs:411:34
    |
411 |         self.entities = unsafe { toxoid_query_entity_list(self.iter) };
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_count` in this scope
   --> crates\toxoid_api\src\ecs.rs:419:25
    |
419 |             let count = toxoid_iter_count(self.iter);
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_field_list` in this scope
   --> crates\toxoid_api\src\ecs.rs:421:31
    |
421 |             let field_slice = toxoid_query_field_list(self.iter, term_index, count as u32) as *mut *const T;
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_count` in this scope
   --> crates\toxoid_api\src\ecs.rs:445:25
    |
445 |             let count = toxoid_iter_count(self.iter);
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_field_list` in this scope
   --> crates\toxoid_api\src\ecs.rs:447:31
    |
447 |             let field_slice = toxoid_query_field_list(self.iter, term_index, count as u32) as *mut *mut T;
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_create` in this scope
   --> crates\toxoid_api\src\ecs.rs:556:34
    |
556 |             query_desc: unsafe { toxoid_query_create() },
    |                                  ^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_iter` in this scope
   --> crates\toxoid_api\src\ecs.rs:562:18
    |
562 |         unsafe { toxoid_query_iter(self.query) }
    |                  ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:574:35
    |
574 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_with` in this scope
   --> crates\toxoid_api\src\ecs.rs:578:38
    |
578 |         self.filter_index = unsafe { toxoid_query_with(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:591:35
    |
591 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_without` in this scope
   --> crates\toxoid_api\src\ecs.rs:596:38
    |
596 |         self.filter_index = unsafe { toxoid_query_without(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:609:35
    |
609 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_with_or` in this scope
   --> crates\toxoid_api\src\ecs.rs:614:38
    |
614 |         self.filter_index = unsafe { toxoid_query_with_or(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:621:38
    |
621 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_order_by` in this scope
   --> crates\toxoid_api\src\ecs.rs:623:13
    |
623 |             toxoid_query_order_by(self.query_desc, component_id, callback);
    |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_build` in this scope
   --> crates\toxoid_api\src\ecs.rs:629:31
    |
629 |         self.query = unsafe { toxoid_query_build(self.query_desc) };
    |                               ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_system_create` in this scope
   --> crates\toxoid_api\src\ecs.rs:648:36
    |
648 |         let system_desc = unsafe { toxoid_system_create(callback) };
    |                                    ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_from_system_desc` in this scope
   --> crates\toxoid_api\src\ecs.rs:649:35
    |
649 |         let query_desc = unsafe { toxoid_query_from_system_desc(system_desc) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:666:35
    |
666 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_with` in this scope
   --> crates\toxoid_api\src\ecs.rs:670:38
    |
670 |         self.filter_index = unsafe { toxoid_query_with(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:683:35
    |
683 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_without` in this scope
   --> crates\toxoid_api\src\ecs.rs:688:38
    |
688 |         self.filter_index = unsafe { toxoid_query_without(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:701:35
    |
701 |                 let id = unsafe { toxoid_component_cache_get(type_id) };
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_with_or` in this scope
   --> crates\toxoid_api\src\ecs.rs:706:38
    |
706 |         self.filter_index = unsafe { toxoid_query_with_or(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
    |                                      ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:713:38
    |
713 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_query_order_by` in this scope
   --> crates\toxoid_api\src\ecs.rs:715:13
    |
715 |             toxoid_query_order_by(self.query_desc, component_id, callback);
    |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_system_build` in this scope
   --> crates\toxoid_api\src\ecs.rs:721:18
    |
721 |         unsafe { toxoid_system_build(self.system_desc) };
    |                  ^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:733:38
    |
733 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_singleton_add` in this scope
   --> crates\toxoid_api\src\ecs.rs:735:13
    |
735 |             toxoid_singleton_add(component_id);
    |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:743:38
    |
743 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_singleton_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:745:23
    |
745 |             let ptr = toxoid_singleton_get(component_id);
    |                       ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:755:38
    |
755 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_singleton_remove` in this scope
   --> crates\toxoid_api\src\ecs.rs:757:13
    |
757 |             toxoid_singleton_remove(component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_delete_entity` in this scope
   --> crates\toxoid_api\src\ecs.rs:763:13
    |
763 |             toxoid_delete_entity(entity.get_id());
    |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_delete_entity` in this scope
   --> crates\toxoid_api\src\ecs.rs:770:13
    |
770 |             toxoid_delete_entity(id);
    |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_prefab_create` in this scope
   --> crates\toxoid_api\src\ecs.rs:783:38
    |
783 |             id: unsafe { combine_u32(toxoid_prefab_create()) },
    |                                      ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:794:38
    |
794 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:796:13
    |
796 |             toxoid_entity_add_component(self.entity.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:803:38
    |
803 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_remove_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:805:13
    |
805 |             toxoid_entity_remove_component(self.entity.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:811:13
    |
811 |             toxoid_entity_add_component(self.entity.id, component);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_tag` in this scope
   --> crates\toxoid_api\src\ecs.rs:817:13
    |
817 |             toxoid_entity_add_tag(self.entity.id, tag);
    |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:829:38
    |
829 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_get_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:831:23
    |
831 |             let ptr = toxoid_entity_get_component(self.entity.id, component_id);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:840:38
    |
840 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_has_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:842:13
    |
842 |             toxoid_entity_has_component(self.entity.id, component_id)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
   --> crates\toxoid_api\src\ecs.rs:848:13
    |
848 |             toxoid_entity_child_of(self.entity.id, parent.get_id());
    |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
   --> crates\toxoid_api\src\ecs.rs:854:13
    |
854 |             toxoid_entity_child_of(child.get_id(), self.entity.id);
    |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_create` in this scope
   --> crates\toxoid_api\src\ecs.rs:889:31
    |
889 |         let entity = unsafe { toxoid_entity_create() };
    |                               ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_set_name` in this scope
   --> crates\toxoid_api\src\ecs.rs:892:18
    |
892 |         unsafe { toxoid_entity_set_name(entity, ""); }
    |                  ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_set_name` in this scope
   --> crates\toxoid_api\src\ecs.rs:901:12
    |
901 |            toxoid_entity_set_name(self.id, name);
    |            ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:926:38
    |
926 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:928:13
    |
928 |             toxoid_entity_add_component(self.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:935:38
    |
935 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_remove_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:937:13
    |
937 |             toxoid_entity_remove_component(self.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:943:13
    |
943 |             toxoid_entity_add_component(self.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_remove_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:949:13
    |
949 |             toxoid_entity_remove_component(self.id, component_id);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_add_tag` in this scope
   --> crates\toxoid_api\src\ecs.rs:955:13
    |
955 |             toxoid_entity_add_tag(self.id, tag);
    |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:968:38
    |
968 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_get_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:970:23
    |
970 |             let ptr = toxoid_entity_get_component(self.id, component_id);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_get` in this scope
   --> crates\toxoid_api\src\ecs.rs:979:38
    |
979 |             let component_id_split = toxoid_component_cache_get(type_hash);
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_has_component` in this scope
   --> crates\toxoid_api\src\ecs.rs:981:13
    |
981 |             toxoid_entity_has_component(self.id, component_id)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
   --> crates\toxoid_api\src\ecs.rs:987:13
    |
987 |             toxoid_entity_child_of(self.id, parent.get_id());
    |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
   --> crates\toxoid_api\src\ecs.rs:993:13
    |
993 |             toxoid_entity_child_of(child.get_id(), self.id);
    |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
   --> crates\toxoid_api\src\ecs.rs:999:13
    |
999 |             toxoid_entity_child_of(self.id, parent);
    |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_entity_child_of` in this scope
    --> crates\toxoid_api\src\ecs.rs:1005:13
     |
1005 |             toxoid_entity_child_of(child, self.id);
     |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_children_init` in this scope
    --> crates\toxoid_api\src\ecs.rs:1015:26
     |
1015 |             let filter = toxoid_filter_children_init(self.get_id());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_iter` in this scope
    --> crates\toxoid_api\src\ecs.rs:1016:22
     |
1016 |             let it = toxoid_filter_iter(filter);
     |                      ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_next` in this scope
    --> crates\toxoid_api\src\ecs.rs:1017:13
     |
1017 |             toxoid_filter_next(it);
     |             ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_entities` in this scope
    --> crates\toxoid_api\src\ecs.rs:1018:28
     |
1018 |             let entities = toxoid_iter_entities(it);
     |                            ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_count` in this scope
    --> crates\toxoid_api\src\ecs.rs:1019:25
     |
1019 |             let count = toxoid_iter_count(it);
     |                         ^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_children_init` in this scope
    --> crates\toxoid_api\src\ecs.rs:1040:26
     |
1040 |             let filter = toxoid_filter_children_init(self.get_id());
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_iter` in this scope
    --> crates\toxoid_api\src\ecs.rs:1041:22
     |
1041 |             let it = toxoid_filter_iter(filter);
     |                      ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_filter_next` in this scope
    --> crates\toxoid_api\src\ecs.rs:1042:19
     |
1042 |             while toxoid_filter_next(it) {
     |                   ^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_iter_entities` in this scope
    --> crates\toxoid_api\src\ecs.rs:1043:32
     |
1043 |                 let entities = toxoid_iter_entities(it);
     |                                ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_delete_entity` in this scope
    --> crates\toxoid_api\src\ecs.rs:1066:9
     |
1066 |         toxoid_delete_entity(entity.get_id());
     |         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_delete_entity` in this scope
    --> crates\toxoid_api\src\ecs.rs:1072:9
     |
1072 |         toxoid_delete_entity(entity.get_id());
     |         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_is_valid` in this scope
    --> crates\toxoid_api\src\ecs.rs:1078:9
     |
1078 |         toxoid_is_valid(entity.get_id())
     |         ^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_is_valid` in this scope
    --> crates\toxoid_api\src\ecs.rs:1084:9
     |
1084 |         toxoid_is_valid(entity.get_id())
     |         ^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_register_tag` in this scope
    --> crates\toxoid_api\src\ecs.rs:1089:14
     |
1089 |     unsafe { toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len()) }
     |              ^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_component_cache_insert` in this scope
    --> crates\toxoid_api\src\ecs.rs:1102:9
     |
1102 |         toxoid_component_cache_insert(type_id, component_id);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_print_i32` in this scope
 --> crates\toxoid_api\src\log.rs:5:9
  |
5 |         toxoid_print_i32(v);
  |         ^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_print_u64` in this scope
  --> crates\toxoid_api\src\log.rs:14:9
   |
14 |         toxoid_print_u64(v);
   |         ^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_print_f32` in this scope
  --> crates\toxoid_api\src\log.rs:20:9
   |
20 |         toxoid_print_f32(v);
   |         ^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_print_f64` in this scope
  --> crates\toxoid_api\src\log.rs:29:9
   |
29 |         toxoid_print_f64(v);
   |         ^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_print_string` in this scope
  --> crates\toxoid_api\src\log.rs:35:9
   |
35 |         toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
   |         ^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_make_c_string` in this scope
   --> crates\toxoid_api\src\components.rs:34:1
    |
34  | / component! {
35  | |     // Singletons
36  | |     GameConfig {
37  | |         resolution_width: u32,
...   |
144 | |     Disconnected {},
145 | | }
    | |_^ not found in this scope
    |
    = note: this error originates in the macro `component` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `toxoid_get_timestamp` in this scope
  --> crates\toxoid_api\src\utils.rs:13:26
   |
13 |     unsafe { combine_f32(toxoid_get_timestamp()) }
   |                          ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_make_c_string` in this scope
  --> crates\toxoid_api\src\utils.rs:17:14
   |
17 |     unsafe { toxoid_make_c_string(string) }
   |              ^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_net_send_components` in this scope
  --> crates\toxoid_api\src\net.rs:50:14
   |
50 |     unsafe { toxoid_net_send_components(split_u64(entity.get_id()), components, event); }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_network_entity_cache_insert` in this scope
  --> crates\toxoid_api\src\net.rs:70:14
   |
70 |     unsafe { toxoid_network_entity_cache_insert(local_id, network_id) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_network_entity_cache_get` in this scope
  --> crates\toxoid_api\src\net.rs:80:14
   |
80 |     unsafe { toxoid_network_entity_cache_get(local_id) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_network_entity_cache_remove` in this scope
  --> crates\toxoid_api\src\net.rs:90:14
   |
90 |     unsafe { toxoid_network_entity_cache_remove(local_id) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `toxoid_deserialize_entity_sync` in this scope
  --> crates\toxoid_api\src\net.rs:94:14
   |
94 |     unsafe { toxoid_deserialize_entity_sync(entity_id, components_serialized) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
For more information about this error, try `rustc --explain E0425`.

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod bindings {
    #![allow(non_camel_case_types)]
    #![allow(improper_ctypes)]
    use crate::ecs::*;
    use core::ffi::c_void;
    pub type ecs_id_t = u64;
    pub type ecs_entity_t = ecs_id_t;
    pub type c_char = i8;
    #[repr(C)]
    pub struct SplitU64 {
        pub high: u32,
        pub low: u32,
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn split_u64(v: u64) -> u64 {
        v
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn combine_u32(split_u64: u64) -> u64 {
        split_u64
    }
    #[repr(C)]
    pub struct SplitF64 {
        pub high: f32,
        pub low: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SplitF64 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SplitF64",
                "high",
                &self.high,
                "low",
                &&self.low,
            )
        }
    }
    pub fn split_f64(value: f64) -> SplitF64 {
        let bits = value.to_bits();
        let high = ((bits >> 32) & 0xFFFFFFFF) as u32;
        let low = (bits & 0xFFFFFFFF) as u32;
        SplitF64 {
            high: f32::from_bits(high),
            low: f32::from_bits(low),
        }
    }
    pub fn combine_f32(split_f64: SplitF64) -> f64 {
        let bits1 = split_f64.high.to_bits() as u64;
        let bits2 = split_f64.high.to_bits() as u64;
        let combined = (bits1 << 32) | bits2;
        f64::from_bits(combined)
    }
}
pub mod ecs {
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
    pub struct Pointer {
        #[serde(skip, default = "default_ptr")]
        pub ptr: *mut c_void,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Pointer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Pointer",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Pointer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Pointer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Pointer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Pointer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        _serde::__private::Ok(Pointer { ptr: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        _serde::__private::Ok(Pointer { ptr: default_ptr() })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Pointer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Pointer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Pointer {
        pub fn new(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    impl Default for Pointer {
        fn default() -> Self {
            Self { ptr: core::ptr::null_mut() }
        }
    }
    impl Clone for Pointer {
        fn clone(&self) -> Self {
            Self { ptr: self.ptr }
        }
    }
    impl Copy for Pointer {}
    impl PartialEq for Pointer {
        fn eq(&self, other: &Self) -> bool {
            self.ptr == other.ptr
        }
    }
    pub struct StringPtr {
        #[serde(skip, default = "default_string")]
        pub ptr: *mut c_char,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for StringPtr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "StringPtr",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for StringPtr {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<StringPtr>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = StringPtr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct StringPtr",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_string();
                        _serde::__private::Ok(StringPtr { ptr: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        _serde::__private::Ok(StringPtr { ptr: default_string() })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "StringPtr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<StringPtr>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl StringPtr {
        pub fn new(rust_str: &str) -> Self {
            Self {
                ptr: unsafe { toxoid_make_c_string(rust_str) },
            }
        }
    }
    impl Default for StringPtr {
        fn default() -> Self {
            Self { ptr: core::ptr::null_mut() }
        }
    }
    impl Clone for StringPtr {
        fn clone(&self) -> Self {
            Self { ptr: self.ptr }
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
            Self { ptr, len }
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
            Self { ptr, len }
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
            Self { ptr, len }
        }
        pub fn create_raw(len: u32) -> *mut u32 {
            unsafe {
                ALLOCATOR.alloc(Layout::array::<u32>(len as usize).unwrap()) as *mut u32
            }
        }
    }
    pub fn create_raw(len: u32) -> *mut u32 {
        unsafe {
            ALLOCATOR.alloc(Layout::array::<u32>(len as usize).unwrap()) as *mut u32
        }
    }
    impl Clone for U32Array {
        fn clone(&self) -> Self {
            let slice = unsafe {
                std::slice::from_raw_parts(self.ptr, self.len as usize)
            };
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
                let other_slice = std::slice::from_raw_parts(
                    other.ptr,
                    other.len as usize,
                );
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
            Self { ptr, len }
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
            Self { ptr, len }
        }
    }
    impl Clone for F32Array {
        fn clone(&self) -> Self {
            let slice = unsafe {
                std::slice::from_raw_parts(self.ptr, self.len as usize)
            };
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
                let other_slice = std::slice::from_raw_parts(
                    other.ptr,
                    other.len as usize,
                );
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
            self
                .filter_index = unsafe {
                toxoid_filter_with(
                    self.filter_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_filter_without(
                    self.filter_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_filter_with_or(
                    self.filter_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
        entities: &'static mut [Entity],
    }
    impl Iter {
        pub fn new() -> Iter {
            Iter {
                iter: core::ptr::null_mut(),
                entities: &mut [],
            }
        }
        pub fn from(iter: *mut c_void) -> Iter {
            Iter { iter, entities: &mut [] }
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
        pub fn entities(&mut self) -> &mut [Entity] {
            self.entities = unsafe { toxoid_query_entity_list(self.iter) };
            self.entities
        }
        pub fn field<T: Default + Component + ComponentType + 'static>(
            &self,
            term_index: i32,
        ) -> &'static [T] {
            unsafe {
                let count = toxoid_iter_count(self.iter);
                let field_slice = toxoid_query_field_list(
                    self.iter,
                    term_index,
                    count as u32,
                ) as *mut *const T;
                let field_slice = core::slice::from_raw_parts(
                    field_slice,
                    count as usize,
                );
                let layout = Layout::array::<T>(count as usize)
                    .expect("Failed to create layout");
                let components_ptr = ALLOCATOR.alloc(layout) as *mut T;
                field_slice
                    .iter()
                    .enumerate()
                    .for_each(|(i, &component_ptr)| {
                        let mut component = T::default();
                        component.set_ptr(component_ptr as *mut c_void);
                        components_ptr.add(i).write(component);
                    });
                let components = core::slice::from_raw_parts(
                    components_ptr,
                    count as usize,
                );
                components
            }
        }
        pub fn field_mut<T: Default + Component + ComponentType + 'static>(
            &self,
            term_index: i32,
        ) -> &'static mut [T] {
            unsafe {
                let count = toxoid_iter_count(self.iter);
                let field_slice = toxoid_query_field_list(
                    self.iter,
                    term_index,
                    count as u32,
                ) as *mut *mut T;
                let field_slice = core::slice::from_raw_parts(
                    field_slice,
                    count as usize,
                );
                let layout = Layout::array::<T>(count as usize)
                    .expect("Failed to create layout");
                let components_ptr = ALLOCATOR.alloc(layout) as *mut T;
                field_slice
                    .iter()
                    .enumerate()
                    .for_each(|(i, &component_ptr)| {
                        let mut component = T::default();
                        component.set_ptr(component_ptr as *mut c_void);
                        components_ptr.add(i).write(component);
                    });
                let components = core::slice::from_raw_parts_mut(
                    components_ptr,
                    count as usize,
                );
                components
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl Drop for Iter {
        fn drop(&mut self) {}
    }
    #[repr(C)]
    pub struct Query {
        query: *mut c_void,
        query_desc: *mut c_void,
        filter_index: u8,
    }
    impl Query {
        pub fn new() -> Query {
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
            self
                .filter_index = unsafe {
                toxoid_query_with(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_query_without(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_query_with_or(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
            self
        }
        pub fn order_by<T: Default + Component + ComponentType + 'static>(
            &mut self,
            callback: extern "C" fn(
                ecs_entity_t,
                *const c_void,
                ecs_entity_t,
                *const c_void,
            ) -> i32,
        ) -> &mut Self {
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
                filter_index: 0,
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
            self
                .filter_index = unsafe {
                toxoid_query_with(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_query_without(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
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
            self
                .filter_index = unsafe {
                toxoid_query_with_or(
                    self.query_desc,
                    self.filter_index,
                    ids_ptr,
                    type_ids.len() as i32,
                )
            };
            self
        }
        pub fn order_by<T: Default + Component + ComponentType + 'static>(
            &mut self,
            callback: extern "C" fn(
                ecs_entity_t,
                *const c_void,
                ecs_entity_t,
                *const c_void,
            ) -> i32,
        ) -> &mut Self {
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
        entity: Entity,
    }
    impl Prefab {
        pub fn new() -> Prefab {
            let entity = Entity {
                id: unsafe { combine_u32(toxoid_prefab_create()) },
                children: &mut [],
            };
            Prefab { entity }
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
    pub struct Entity {
        pub id: ecs_id_t,
        pub children: &'static mut [Entity],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Entity {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Entity",
                "id",
                &self.id,
                "children",
                &&self.children,
            )
        }
    }
    impl Entity {
        pub fn new() -> Entity {
            let entity = unsafe { toxoid_entity_create() };
            unsafe {
                toxoid_entity_set_name(entity, "");
            }
            Entity {
                id: entity,
                children: &mut [],
            }
        }
        pub fn set_name(&self, name: &str) {
            unsafe {
                toxoid_entity_set_name(self.id, name);
            }
        }
        pub fn from_id(id: ecs_id_t) -> Entity {
            Entity { id, children: &mut [] }
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
            ::core::panicking::panic("not implemented")
        }
        pub fn children(&mut self) -> &mut [Entity] {
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
                        entities_ptr
                            .add(i)
                            .write(Entity {
                                id: *entity_id,
                                children: &mut [],
                            });
                    });
                let entities = core::slice::from_raw_parts_mut(
                    entities_ptr,
                    count as usize,
                );
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
                                children: &mut [],
                            };
                            cb(e);
                        });
                }
            }
        }
        pub fn drop(&mut self) {
            unsafe {
                ALLOCATOR
                    .dealloc(
                        self.children.as_ptr() as *mut u8,
                        Layout::array::<Entity>(self.children.len() as usize).unwrap(),
                    );
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
        unsafe { toxoid_is_valid(entity.get_id()) }
    }
    pub fn is_valid_mut(entity: &mut Entity) -> bool {
        unsafe { toxoid_is_valid(entity.get_id()) }
    }
    pub fn register_tag(name: &str) -> ecs_entity_t {
        unsafe { toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len()) }
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn cache_component_ecs(type_id: u64, component_id: u64) {
        unsafe {
            toxoid_component_cache_insert(type_id, component_id);
        }
    }
    impl<Component1: Default + Component + ComponentType + 'static> ComponentTuple
    for (Component1,) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
    > ComponentTuple for (Component1, Component2) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
    > ComponentTuple for (Component1, Component2, Component3) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
    > ComponentTuple for (Component1, Component2, Component3, Component4) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
    > ComponentTuple for (Component1, Component2, Component3, Component4, Component5) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (Component1, Component2, Component3, Component4, Component5, Component6) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(&[(), (), (), (), (), (), (), (), (), (), ()]);
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
        Component16: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
        Component16,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component16::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
        Component16: Default + Component + ComponentType + 'static,
        Component17: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
        Component16,
        Component17,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component16::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component17::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
        Component16: Default + Component + ComponentType + 'static,
        Component17: Default + Component + ComponentType + 'static,
        Component18: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
        Component16,
        Component17,
        Component18,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                    ],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component16::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component17::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component18::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
        Component16: Default + Component + ComponentType + 'static,
        Component17: Default + Component + ComponentType + 'static,
        Component18: Default + Component + ComponentType + 'static,
        Component19: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
        Component16,
        Component17,
        Component18,
        Component19,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                    ],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component16::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component17::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component18::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component19::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
    impl<
        Component1: Default + Component + ComponentType + 'static,
        Component2: Default + Component + ComponentType + 'static,
        Component3: Default + Component + ComponentType + 'static,
        Component4: Default + Component + ComponentType + 'static,
        Component5: Default + Component + ComponentType + 'static,
        Component6: Default + Component + ComponentType + 'static,
        Component7: Default + Component + ComponentType + 'static,
        Component8: Default + Component + ComponentType + 'static,
        Component9: Default + Component + ComponentType + 'static,
        Component10: Default + Component + ComponentType + 'static,
        Component11: Default + Component + ComponentType + 'static,
        Component12: Default + Component + ComponentType + 'static,
        Component13: Default + Component + ComponentType + 'static,
        Component14: Default + Component + ComponentType + 'static,
        Component15: Default + Component + ComponentType + 'static,
        Component16: Default + Component + ComponentType + 'static,
        Component17: Default + Component + ComponentType + 'static,
        Component18: Default + Component + ComponentType + 'static,
        Component19: Default + Component + ComponentType + 'static,
        Component20: Default + Component + ComponentType + 'static,
    > ComponentTuple
    for (
        Component1,
        Component2,
        Component3,
        Component4,
        Component5,
        Component6,
        Component7,
        Component8,
        Component9,
        Component10,
        Component11,
        Component12,
        Component13,
        Component14,
        Component15,
        Component16,
        Component17,
        Component18,
        Component19,
        Component20,
    ) {
        #[allow(unused_assignments)]
        fn get_type_ids() -> &'static [u64] {
            unsafe {
                let count = <[()]>::len(
                    &[
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                        (),
                    ],
                );
                let layout = Layout::array::<u64>(count).unwrap();
                let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64;
                let mut i = 0;
                type_ids_ptr.add(i).write(Component1::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component2::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component3::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component4::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component5::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component6::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component7::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component8::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component9::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component10::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component11::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component12::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component13::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component14::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component15::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component16::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component17::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component18::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component19::get_hash());
                i += 1;
                type_ids_ptr.add(i).write(Component20::get_hash());
                i += 1;
                let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                type_ids
            }
        }
    }
}
pub mod globals {
    #![allow(improper_ctypes)]
    extern crate alloc;
    use core::alloc::{GlobalAlloc, Layout};
    extern "C" {
        fn host_alloc(layout: Layout) -> *mut u8;
        fn host_dealloc(ptr: *mut u8, layout: Layout);
    }
    pub struct GuestAllocator;
    unsafe impl GlobalAlloc for GuestAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            host_alloc(layout)
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            host_dealloc(ptr, layout)
        }
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub static ALLOCATOR: std::alloc::System = std::alloc::System;
    const _: () = {
        #[rustc_std_internal_symbol]
        unsafe fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc(
                &ALLOCATOR,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) -> () {
            ::core::alloc::GlobalAlloc::dealloc(
                &ALLOCATOR,
                ptr,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_realloc(
            ptr: *mut u8,
            size: usize,
            align: usize,
            new_size: usize,
        ) -> *mut u8 {
            ::core::alloc::GlobalAlloc::realloc(
                &ALLOCATOR,
                ptr,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
                new_size,
            )
        }
        #[rustc_std_internal_symbol]
        unsafe fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc_zeroed(
                &ALLOCATOR,
                ::core::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }
    };
    #[no_mangle]
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub extern "C" fn allocate(size: usize) -> *mut u8 {
        let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
        unsafe { std::alloc::alloc(layout) }
    }
    #[no_mangle]
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub extern "C" fn deallocate(ptr: *mut u8, size: usize) {
        let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
        unsafe { std::alloc::dealloc(ptr, layout) }
    }
}
pub mod log {
    use crate::bindings::*;
    pub fn print_i32(v: i32) {
        unsafe {
            toxoid_print_i32(v);
        }
    }
    pub fn print_u64(v: u64) {
        unsafe {
            #[cfg(not(all(target_arch = "wasm32", target_os = "emscripten")))]
            toxoid_print_u64(v);
        }
    }
    pub fn print_f32(v: f32) {
        unsafe {
            toxoid_print_f32(v);
        }
    }
    pub fn print_f64(v: f64) {
        unsafe {
            #[cfg(not(all(target_arch = "wasm32", target_os = "emscripten")))]
            toxoid_print_f64(v);
        }
    }
    pub fn print_string(v: &str) {
        unsafe {
            toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
        }
    }
}
pub mod components {
    use serde::{Deserialize, Serialize};
    use toxoid_api_macro::component;
    use crate::*;
    pub enum DirectionEnum {
        Up = 0,
        Down = 1,
        Left = 2,
        Right = 3,
    }
    pub enum KeyCode {
        Up = 38,
        Down = 40,
        Left = 37,
        Right = 39,
    }
    pub enum BlendModes {
        None = 0,
        Blend = 1,
        Add = 2,
        Mod = 3,
        Multiply = 4,
    }
    #[repr(C)]
    pub struct GameConfig {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub resolution_width: u32,
        pub resolution_height: u32,
        pub scale_factor: f32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GameConfig {
        #[inline]
        fn clone(&self) -> GameConfig {
            GameConfig {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                resolution_width: ::core::clone::Clone::clone(&self.resolution_width),
                resolution_height: ::core::clone::Clone::clone(&self.resolution_height),
                scale_factor: ::core::clone::Clone::clone(&self.scale_factor),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GameConfig {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GameConfig {
        #[inline]
        fn eq(&self, other: &GameConfig) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.resolution_width == other.resolution_width
                && self.resolution_height == other.resolution_height
                && self.scale_factor == other.scale_factor
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for GameConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "GameConfig",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "resolution_width",
                    &self.resolution_width,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "resolution_height",
                    &self.resolution_height,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "scale_factor",
                    &self.scale_factor,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for GameConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            4u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "resolution_width" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            "resolution_height" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            "scale_factor" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"resolution_width" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"resolution_height" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            b"scale_factor" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GameConfig>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GameConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct GameConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct GameConfig with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct GameConfig with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct GameConfig with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct GameConfig with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct GameConfig with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(GameConfig {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            resolution_width: __field3,
                            resolution_height: __field4,
                            scale_factor: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<f32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resolution_width",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resolution_height",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "scale_factor",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("resolution_width")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("resolution_height")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("scale_factor")?
                            }
                        };
                        _serde::__private::Ok(GameConfig {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            resolution_width: __field3,
                            resolution_height: __field4,
                            scale_factor: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "resolution_width",
                    "resolution_height",
                    "scale_factor",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GameConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GameConfig>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for GameConfig {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                resolution_width: u32::default(),
                resolution_height: u32::default(),
                scale_factor: f32::default(),
            }
        }
    }
    impl GameConfig {
        pub fn get_resolution_width(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 0u32) }
        }
        pub fn set_resolution_width(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 0u32, value);
            }
        }
        pub fn get_resolution_height(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 4u32) }
        }
        pub fn set_resolution_height(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 4u32, value);
            }
        }
        pub fn get_scale_factor(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 8u32) }
        }
        pub fn set_scale_factor(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 8u32, value);
            }
        }
    }
    impl ComponentType for GameConfig {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "GameConfig",
                    &["resolution_width", "resolution_height", "scale_factor"],
                    &[2u8, 2u8, 8u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(2075606517822088261u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            2075606517822088261u64
        }
        fn get_name() -> &'static str {
            "GameConfig"
        }
    }
    impl Component for GameConfig {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("GameConfig"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct KeyboardInput {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub up: bool,
        pub down: bool,
        pub left: bool,
        pub right: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeyboardInput {
        #[inline]
        fn clone(&self) -> KeyboardInput {
            KeyboardInput {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                up: ::core::clone::Clone::clone(&self.up),
                down: ::core::clone::Clone::clone(&self.down),
                left: ::core::clone::Clone::clone(&self.left),
                right: ::core::clone::Clone::clone(&self.right),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for KeyboardInput {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for KeyboardInput {
        #[inline]
        fn eq(&self, other: &KeyboardInput) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.up == other.up && self.down == other.down
                && self.left == other.left && self.right == other.right
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for KeyboardInput {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "KeyboardInput",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "up",
                    &self.up,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "down",
                    &self.down,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "left",
                    &self.left,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "right",
                    &self.right,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KeyboardInput {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            4u64 => _serde::__private::Ok(__Field::__field5),
                            5u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "up" => _serde::__private::Ok(__Field::__field3),
                            "down" => _serde::__private::Ok(__Field::__field4),
                            "left" => _serde::__private::Ok(__Field::__field5),
                            "right" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"up" => _serde::__private::Ok(__Field::__field3),
                            b"down" => _serde::__private::Ok(__Field::__field4),
                            b"left" => _serde::__private::Ok(__Field::__field5),
                            b"right" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<KeyboardInput>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KeyboardInput;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct KeyboardInput",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct KeyboardInput with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(KeyboardInput {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            up: __field3,
                            down: __field4,
                            left: __field5,
                            right: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("down"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("left"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("right"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("up")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("down")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("left")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("right")?
                            }
                        };
                        _serde::__private::Ok(KeyboardInput {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            up: __field3,
                            down: __field4,
                            left: __field5,
                            right: __field6,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "up",
                    "down",
                    "left",
                    "right",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "KeyboardInput",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<KeyboardInput>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for KeyboardInput {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                up: bool::default(),
                down: bool::default(),
                left: bool::default(),
                right: bool::default(),
            }
        }
    }
    impl KeyboardInput {
        pub fn get_up(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 0u32) }
        }
        pub fn set_up(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 0u32, value);
            }
        }
        pub fn get_down(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 1u32) }
        }
        pub fn set_down(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 1u32, value);
            }
        }
        pub fn get_left(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 2u32) }
        }
        pub fn set_left(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 2u32, value);
            }
        }
        pub fn get_right(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 3u32) }
        }
        pub fn set_right(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 3u32, value);
            }
        }
    }
    impl ComponentType for KeyboardInput {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "KeyboardInput",
                    &["up", "down", "left", "right"],
                    &[10u8, 10u8, 10u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(13148766811431805240u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            13148766811431805240u64
        }
        fn get_name() -> &'static str {
            "KeyboardInput"
        }
    }
    impl Component for KeyboardInput {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("KeyboardInput"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct SpineInstance {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub instance: Pointer,
        #[serde(skip)]
        pub ctx: Pointer,
        pub instantiated: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SpineInstance {
        #[inline]
        fn clone(&self) -> SpineInstance {
            SpineInstance {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                instance: ::core::clone::Clone::clone(&self.instance),
                ctx: ::core::clone::Clone::clone(&self.ctx),
                instantiated: ::core::clone::Clone::clone(&self.instantiated),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SpineInstance {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SpineInstance {
        #[inline]
        fn eq(&self, other: &SpineInstance) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.instance == other.instance
                && self.ctx == other.ctx && self.instantiated == other.instantiated
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SpineInstance {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "SpineInstance",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "instantiated",
                    &self.instantiated,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SpineInstance {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "instantiated" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"instantiated" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SpineInstance>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SpineInstance;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SpineInstance",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct SpineInstance with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct SpineInstance with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = _serde::__private::Default::default();
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct SpineInstance with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(SpineInstance {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            instance: __field3,
                            ctx: __field4,
                            instantiated: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "instantiated",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("instantiated")?
                            }
                        };
                        _serde::__private::Ok(SpineInstance {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            instance: _serde::__private::Default::default(),
                            ctx: _serde::__private::Default::default(),
                            instantiated: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "instantiated",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SpineInstance",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SpineInstance>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for SpineInstance {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                instance: Pointer::default(),
                ctx: Pointer::default(),
                instantiated: bool::default(),
            }
        }
    }
    impl SpineInstance {
        pub fn get_instance(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_instance(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_ctx(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 8u32),
                }
            }
        }
        pub fn set_ctx(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 8u32, value.ptr);
            }
        }
        pub fn get_instantiated(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 16u32) }
        }
        pub fn set_instantiated(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 16u32, value);
            }
        }
    }
    impl ComponentType for SpineInstance {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "SpineInstance",
                    &["instance", "ctx", "instantiated"],
                    &[14u8, 14u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(17076370598395513423u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            17076370598395513423u64
        }
        fn get_name() -> &'static str {
            "SpineInstance"
        }
    }
    impl Component for SpineInstance {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("SpineInstance"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Position {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub x: u32,
        pub y: u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Position {
        #[inline]
        fn clone(&self) -> Position {
            Position {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Position {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Position {
        #[inline]
        fn eq(&self, other: &Position) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.x == other.x && self.y == other.y
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Position {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Position",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "x",
                    &self.x,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "y",
                    &self.y,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Position {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "x" => _serde::__private::Ok(__Field::__field3),
                            "y" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"x" => _serde::__private::Ok(__Field::__field3),
                            b"y" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Position>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Position;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Position",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Position with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Position with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Position with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Position with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Position {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            x: __field3,
                            y: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("x"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("y"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("x")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("y")?
                            }
                        };
                        _serde::__private::Ok(Position {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            x: __field3,
                            y: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id", "x", "y"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Position",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Position>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Position {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                x: u32::default(),
                y: u32::default(),
            }
        }
    }
    impl Position {
        pub fn get_x(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 0u32) }
        }
        pub fn set_x(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 0u32, value);
            }
        }
        pub fn get_y(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 4u32) }
        }
        pub fn set_y(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 4u32, value);
            }
        }
    }
    impl ComponentType for Position {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Position", &["x", "y"], &[2u8, 2u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(13934449531345340970u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            13934449531345340970u64
        }
        fn get_name() -> &'static str {
            "Position"
        }
    }
    impl Component for Position {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Position"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Velocity {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub dx: f32,
        pub dy: f32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Velocity {
        #[inline]
        fn clone(&self) -> Velocity {
            Velocity {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                dx: ::core::clone::Clone::clone(&self.dx),
                dy: ::core::clone::Clone::clone(&self.dy),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Velocity {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Velocity {
        #[inline]
        fn eq(&self, other: &Velocity) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.dx == other.dx && self.dy == other.dy
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Velocity {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Velocity",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dx",
                    &self.dx,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dy",
                    &self.dy,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Velocity {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "dx" => _serde::__private::Ok(__Field::__field3),
                            "dy" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"dx" => _serde::__private::Ok(__Field::__field3),
                            b"dy" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Velocity>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Velocity;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Velocity",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Velocity with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Velocity with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Velocity with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Velocity with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Velocity {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            dx: __field3,
                            dy: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<f32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("dx"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("dy"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("dx")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("dy")?
                            }
                        };
                        _serde::__private::Ok(Velocity {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            dx: __field3,
                            dy: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id", "dx", "dy"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Velocity",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Velocity>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Velocity {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                dx: f32::default(),
                dy: f32::default(),
            }
        }
    }
    impl Velocity {
        pub fn get_dx(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 0u32) }
        }
        pub fn set_dx(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 0u32, value);
            }
        }
        pub fn get_dy(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 4u32) }
        }
        pub fn set_dy(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 4u32, value);
            }
        }
    }
    impl ComponentType for Velocity {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Velocity", &["dx", "dy"], &[8u8, 8u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(15715703694425123250u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            15715703694425123250u64
        }
        fn get_name() -> &'static str {
            "Velocity"
        }
    }
    impl Component for Velocity {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Velocity"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Direction {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub direction: u8,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Direction {
        #[inline]
        fn clone(&self) -> Direction {
            Direction {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                direction: ::core::clone::Clone::clone(&self.direction),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Direction {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Direction {
        #[inline]
        fn eq(&self, other: &Direction) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.direction == other.direction
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Direction {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Direction",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "direction",
                    &self.direction,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Direction {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "direction" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"direction" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Direction>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Direction;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Direction",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Direction with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Direction with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Direction with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Direction {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            direction: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u8> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "direction",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("direction")?
                            }
                        };
                        _serde::__private::Ok(Direction {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            direction: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "direction",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Direction",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Direction>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Direction {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                direction: u8::default(),
            }
        }
    }
    impl Direction {
        pub fn get_direction(&self) -> u8 {
            unsafe { toxoid_component_get_member_u8(self.ptr, 0u32) }
        }
        pub fn set_direction(&mut self, value: u8) {
            unsafe {
                toxoid_component_set_member_u8(self.ptr, 0u32, value);
            }
        }
    }
    impl ComponentType for Direction {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Direction", &["direction"], &[0u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(15149798974371151882u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            15149798974371151882u64
        }
        fn get_name() -> &'static str {
            "Direction"
        }
    }
    impl Component for Direction {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Direction"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Size {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub width: i32,
        pub height: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Size {
        #[inline]
        fn clone(&self) -> Size {
            Size {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                width: ::core::clone::Clone::clone(&self.width),
                height: ::core::clone::Clone::clone(&self.height),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Size {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Size {
        #[inline]
        fn eq(&self, other: &Size) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.width == other.width
                && self.height == other.height
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Size {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Size",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "width",
                    &self.width,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "height",
                    &self.height,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Size {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "width" => _serde::__private::Ok(__Field::__field3),
                            "height" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"width" => _serde::__private::Ok(__Field::__field3),
                            b"height" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Size>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Size;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Size",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Size with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Size with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Size with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Size with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Size {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            width: __field3,
                            height: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<i32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("width"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("height"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("width")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("height")?
                            }
                        };
                        _serde::__private::Ok(Size {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            width: __field3,
                            height: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "width",
                    "height",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Size",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Size>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Size {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                width: i32::default(),
                height: i32::default(),
            }
        }
    }
    impl Size {
        pub fn get_width(&self) -> i32 {
            unsafe { toxoid_component_get_member_i32(self.ptr, 0u32) }
        }
        pub fn set_width(&mut self, value: i32) {
            unsafe {
                toxoid_component_set_member_i32(self.ptr, 0u32, value);
            }
        }
        pub fn get_height(&self) -> i32 {
            unsafe { toxoid_component_get_member_i32(self.ptr, 4u32) }
        }
        pub fn set_height(&mut self, value: i32) {
            unsafe {
                toxoid_component_set_member_i32(self.ptr, 4u32, value);
            }
        }
    }
    impl ComponentType for Size {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Size", &["width", "height"], &[6u8, 6u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(8680854466515771932u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            8680854466515771932u64
        }
        fn get_name() -> &'static str {
            "Size"
        }
    }
    impl Component for Size {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe { toxoid_component_lookup(toxoid_make_c_string("Size")) })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Color {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        fn clone(&self) -> Color {
            Color {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                r: ::core::clone::Clone::clone(&self.r),
                g: ::core::clone::Clone::clone(&self.g),
                b: ::core::clone::Clone::clone(&self.b),
                a: ::core::clone::Clone::clone(&self.a),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Color {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, other: &Color) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.r == other.r && self.g == other.g
                && self.b == other.b && self.a == other.a
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Color {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Color",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "r",
                    &self.r,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "g",
                    &self.g,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "b",
                    &self.b,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "a",
                    &self.a,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Color {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            4u64 => _serde::__private::Ok(__Field::__field5),
                            5u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "r" => _serde::__private::Ok(__Field::__field3),
                            "g" => _serde::__private::Ok(__Field::__field4),
                            "b" => _serde::__private::Ok(__Field::__field5),
                            "a" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"r" => _serde::__private::Ok(__Field::__field3),
                            b"g" => _serde::__private::Ok(__Field::__field4),
                            b"b" => _serde::__private::Ok(__Field::__field5),
                            b"a" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Color>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Color;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Color",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Color with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Color {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            r: __field3,
                            g: __field4,
                            b: __field5,
                            a: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<f32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("r"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("g"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("r")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("g")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("b")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("a")?
                            }
                        };
                        _serde::__private::Ok(Color {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            r: __field3,
                            g: __field4,
                            b: __field5,
                            a: __field6,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "r",
                    "g",
                    "b",
                    "a",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Color",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Color>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Color {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                r: f32::default(),
                g: f32::default(),
                b: f32::default(),
                a: f32::default(),
            }
        }
    }
    impl Color {
        pub fn get_r(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 0u32) }
        }
        pub fn set_r(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 0u32, value);
            }
        }
        pub fn get_g(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 4u32) }
        }
        pub fn set_g(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 4u32, value);
            }
        }
        pub fn get_b(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 8u32) }
        }
        pub fn set_b(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 8u32, value);
            }
        }
        pub fn get_a(&self) -> f32 {
            unsafe { toxoid_component_get_member_f32(self.ptr, 12u32) }
        }
        pub fn set_a(&mut self, value: f32) {
            unsafe {
                toxoid_component_set_member_f32(self.ptr, 12u32, value);
            }
        }
    }
    impl ComponentType for Color {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Color",
                    &["r", "g", "b", "a"],
                    &[8u8, 8u8, 8u8, 8u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(3769135706557701272u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            3769135706557701272u64
        }
        fn get_name() -> &'static str {
            "Color"
        }
    }
    impl Component for Color {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Color"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Sprite {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub filename: StringPtr,
        #[serde(skip)]
        pub sprite: Pointer,
        #[serde(skip)]
        pub data: Pointer,
        pub data_size: i32,
        pub renderable: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Sprite {
        #[inline]
        fn clone(&self) -> Sprite {
            Sprite {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                filename: ::core::clone::Clone::clone(&self.filename),
                sprite: ::core::clone::Clone::clone(&self.sprite),
                data: ::core::clone::Clone::clone(&self.data),
                data_size: ::core::clone::Clone::clone(&self.data_size),
                renderable: ::core::clone::Clone::clone(&self.renderable),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Sprite {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Sprite {
        #[inline]
        fn eq(&self, other: &Sprite) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.filename == other.filename
                && self.sprite == other.sprite && self.data == other.data
                && self.data_size == other.data_size
                && self.renderable == other.renderable
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Sprite {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Sprite",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "filename",
                    &self.filename,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data_size",
                    &self.data_size,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "renderable",
                    &self.renderable,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Sprite {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field6,
                    __field7,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field6),
                            4u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "filename" => _serde::__private::Ok(__Field::__field3),
                            "data_size" => _serde::__private::Ok(__Field::__field6),
                            "renderable" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"filename" => _serde::__private::Ok(__Field::__field3),
                            b"data_size" => _serde::__private::Ok(__Field::__field6),
                            b"renderable" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Sprite>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Sprite;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Sprite",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Sprite with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Sprite with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            StringPtr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Sprite with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = _serde::__private::Default::default();
                        let __field5 = _serde::__private::Default::default();
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Sprite with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Sprite with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Sprite {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            filename: __field3,
                            sprite: __field4,
                            data: __field5,
                            data_size: __field6,
                            renderable: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<StringPtr> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "filename",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<StringPtr>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data_size",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "renderable",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("filename")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("data_size")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("renderable")?
                            }
                        };
                        _serde::__private::Ok(Sprite {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            filename: __field3,
                            sprite: _serde::__private::Default::default(),
                            data: _serde::__private::Default::default(),
                            data_size: __field6,
                            renderable: __field7,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "filename",
                    "data_size",
                    "renderable",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Sprite",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Sprite>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Sprite {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                filename: StringPtr::default(),
                sprite: Pointer::default(),
                data: Pointer::default(),
                data_size: i32::default(),
                renderable: bool::default(),
            }
        }
    }
    impl Sprite {
        pub fn get_filename(&self) -> &str {
            unsafe {
                let member_ptr = toxoid_component_get_member_ptr(self.ptr, 0u32);
                let c_str: &core::ffi::CStr = unsafe {
                    core::ffi::CStr::from_ptr(member_ptr as *const i8)
                };
                let string_ptr: &str = c_str.to_str().unwrap();
                string_ptr
            }
        }
        pub fn set_filename(&mut self, value: StringPtr) {
            unsafe {
                toxoid_component_set_member_ptr(
                    self.ptr,
                    0u32,
                    value.ptr as *mut core::ffi::c_void,
                );
            }
        }
        pub fn get_sprite(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 8u32),
                }
            }
        }
        pub fn set_sprite(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 8u32, value.ptr);
            }
        }
        pub fn get_data(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 16u32),
                }
            }
        }
        pub fn set_data(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 16u32, value.ptr);
            }
        }
        pub fn get_data_size(&self) -> i32 {
            unsafe { toxoid_component_get_member_i32(self.ptr, 24u32) }
        }
        pub fn set_data_size(&mut self, value: i32) {
            unsafe {
                toxoid_component_set_member_i32(self.ptr, 24u32, value);
            }
        }
        pub fn get_renderable(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 28u32) }
        }
        pub fn set_renderable(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 28u32, value);
            }
        }
    }
    impl ComponentType for Sprite {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Sprite",
                    &["filename", "sprite", "data", "data_size", "renderable"],
                    &[11u8, 14u8, 14u8, 6u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(17555429528309779676u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            17555429528309779676u64
        }
        fn get_name() -> &'static str {
            "Sprite"
        }
    }
    impl Component for Sprite {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Sprite"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct BlendMode {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub mode: u8,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BlendMode {
        #[inline]
        fn clone(&self) -> BlendMode {
            BlendMode {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                mode: ::core::clone::Clone::clone(&self.mode),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BlendMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BlendMode {
        #[inline]
        fn eq(&self, other: &BlendMode) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.mode == other.mode
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BlendMode {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "BlendMode",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mode",
                    &self.mode,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BlendMode {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "mode" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"mode" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<BlendMode>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BlendMode;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct BlendMode",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct BlendMode with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct BlendMode with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct BlendMode with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(BlendMode {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            mode: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u8> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("mode")?
                            }
                        };
                        _serde::__private::Ok(BlendMode {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            mode: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id", "mode"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BlendMode",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<BlendMode>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for BlendMode {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                mode: u8::default(),
            }
        }
    }
    impl BlendMode {
        pub fn get_mode(&self) -> u8 {
            unsafe { toxoid_component_get_member_u8(self.ptr, 0u32) }
        }
        pub fn set_mode(&mut self, value: u8) {
            unsafe {
                toxoid_component_set_member_u8(self.ptr, 0u32, value);
            }
        }
    }
    impl ComponentType for BlendMode {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("BlendMode", &["mode"], &[0u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(12358569567954146443u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            12358569567954146443u64
        }
        fn get_name() -> &'static str {
            "BlendMode"
        }
    }
    impl Component for BlendMode {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("BlendMode"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct RenderTarget {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub render_target: Pointer,
        pub flip_y: bool,
        pub z_index: u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RenderTarget {
        #[inline]
        fn clone(&self) -> RenderTarget {
            RenderTarget {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                render_target: ::core::clone::Clone::clone(&self.render_target),
                flip_y: ::core::clone::Clone::clone(&self.flip_y),
                z_index: ::core::clone::Clone::clone(&self.z_index),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RenderTarget {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RenderTarget {
        #[inline]
        fn eq(&self, other: &RenderTarget) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.render_target == other.render_target
                && self.flip_y == other.flip_y && self.z_index == other.z_index
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RenderTarget {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "RenderTarget",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "flip_y",
                    &self.flip_y,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "z_index",
                    &self.z_index,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RenderTarget {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            3u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "flip_y" => _serde::__private::Ok(__Field::__field4),
                            "z_index" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"flip_y" => _serde::__private::Ok(__Field::__field4),
                            b"z_index" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RenderTarget>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RenderTarget;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RenderTarget",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RenderTarget with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RenderTarget with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct RenderTarget with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct RenderTarget with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RenderTarget {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            render_target: __field3,
                            flip_y: __field4,
                            z_index: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("flip_y"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "z_index",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("flip_y")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("z_index")?
                            }
                        };
                        _serde::__private::Ok(RenderTarget {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            render_target: _serde::__private::Default::default(),
                            flip_y: __field4,
                            z_index: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "flip_y",
                    "z_index",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RenderTarget",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RenderTarget>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for RenderTarget {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                render_target: Pointer::default(),
                flip_y: bool::default(),
                z_index: u32::default(),
            }
        }
    }
    impl RenderTarget {
        pub fn get_render_target(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_render_target(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_flip_y(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 8u32) }
        }
        pub fn set_flip_y(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 8u32, value);
            }
        }
        pub fn get_z_index(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 9u32) }
        }
        pub fn set_z_index(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 9u32, value);
            }
        }
    }
    impl ComponentType for RenderTarget {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "RenderTarget",
                    &["render_target", "flip_y", "z_index"],
                    &[14u8, 10u8, 2u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(713357859068076624u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            713357859068076624u64
        }
        fn get_name() -> &'static str {
            "RenderTarget"
        }
    }
    impl Component for RenderTarget {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("RenderTarget"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Networked {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub network_id: u64,
        pub entity_id: u64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Networked {
        #[inline]
        fn clone(&self) -> Networked {
            Networked {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                network_id: ::core::clone::Clone::clone(&self.network_id),
                entity_id: ::core::clone::Clone::clone(&self.entity_id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Networked {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Networked {
        #[inline]
        fn eq(&self, other: &Networked) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.network_id == other.network_id
                && self.entity_id == other.entity_id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Networked {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Networked",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "network_id",
                    &self.network_id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "entity_id",
                    &self.entity_id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Networked {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "network_id" => _serde::__private::Ok(__Field::__field3),
                            "entity_id" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"network_id" => _serde::__private::Ok(__Field::__field3),
                            b"entity_id" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Networked>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Networked;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Networked",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Networked with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Networked with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Networked with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Networked with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Networked {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            network_id: __field3,
                            entity_id: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u64> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "network_id",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "entity_id",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("network_id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("entity_id")?
                            }
                        };
                        _serde::__private::Ok(Networked {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            network_id: __field3,
                            entity_id: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "network_id",
                    "entity_id",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Networked",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Networked>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Networked {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                network_id: u64::default(),
                entity_id: u64::default(),
            }
        }
    }
    impl Networked {
        pub fn get_network_id(&self) -> u64 {
            unsafe { combine_u32(toxoid_component_get_member_u64(self.ptr, 0u32)) }
        }
        pub fn set_network_id(&mut self, value: u64) {
            unsafe {
                toxoid_component_set_member_u64(self.ptr, 0u32, split_u64(value));
            }
        }
        pub fn get_entity_id(&self) -> u64 {
            unsafe { combine_u32(toxoid_component_get_member_u64(self.ptr, 8u32)) }
        }
        pub fn set_entity_id(&mut self, value: u64) {
            unsafe {
                toxoid_component_set_member_u64(self.ptr, 8u32, split_u64(value));
            }
        }
    }
    impl ComponentType for Networked {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Networked",
                    &["network_id", "entity_id"],
                    &[3u8, 3u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(2977499015570422124u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            2977499015570422124u64
        }
        fn get_name() -> &'static str {
            "Networked"
        }
    }
    impl Component for Networked {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Networked"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct WebSocket {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub socket: Pointer,
        pub connected: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WebSocket {
        #[inline]
        fn clone(&self) -> WebSocket {
            WebSocket {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                socket: ::core::clone::Clone::clone(&self.socket),
                connected: ::core::clone::Clone::clone(&self.connected),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for WebSocket {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for WebSocket {
        #[inline]
        fn eq(&self, other: &WebSocket) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.socket == other.socket
                && self.connected == other.connected
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for WebSocket {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "WebSocket",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "connected",
                    &self.connected,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WebSocket {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "connected" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"connected" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WebSocket>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WebSocket;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WebSocket",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WebSocket with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct WebSocket with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct WebSocket with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(WebSocket {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            socket: __field3,
                            connected: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "connected",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("connected")?
                            }
                        };
                        _serde::__private::Ok(WebSocket {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            socket: _serde::__private::Default::default(),
                            connected: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "connected",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WebSocket",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WebSocket>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for WebSocket {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                socket: Pointer::default(),
                connected: bool::default(),
            }
        }
    }
    impl WebSocket {
        pub fn get_socket(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_socket(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_connected(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 8u32) }
        }
        pub fn set_connected(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 8u32, value);
            }
        }
    }
    impl ComponentType for WebSocket {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "WebSocket",
                    &["socket", "connected"],
                    &[14u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(5802015162306419344u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            5802015162306419344u64
        }
        fn get_name() -> &'static str {
            "WebSocket"
        }
    }
    impl Component for WebSocket {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("WebSocket"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Atlas {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub atlas: Pointer,
        pub filename: StringPtr,
        #[serde(skip)]
        pub data: Pointer,
        pub data_size: i32,
        pub loaded: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Atlas {
        #[inline]
        fn clone(&self) -> Atlas {
            Atlas {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                atlas: ::core::clone::Clone::clone(&self.atlas),
                filename: ::core::clone::Clone::clone(&self.filename),
                data: ::core::clone::Clone::clone(&self.data),
                data_size: ::core::clone::Clone::clone(&self.data_size),
                loaded: ::core::clone::Clone::clone(&self.loaded),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Atlas {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Atlas {
        #[inline]
        fn eq(&self, other: &Atlas) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.atlas == other.atlas
                && self.filename == other.filename && self.data == other.data
                && self.data_size == other.data_size && self.loaded == other.loaded
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Atlas {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Atlas",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "filename",
                    &self.filename,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data_size",
                    &self.data_size,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "loaded",
                    &self.loaded,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Atlas {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __field6,
                    __field7,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            3u64 => _serde::__private::Ok(__Field::__field6),
                            4u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "filename" => _serde::__private::Ok(__Field::__field4),
                            "data_size" => _serde::__private::Ok(__Field::__field6),
                            "loaded" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"filename" => _serde::__private::Ok(__Field::__field4),
                            b"data_size" => _serde::__private::Ok(__Field::__field6),
                            b"loaded" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Atlas>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Atlas;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Atlas",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Atlas with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Atlas with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            StringPtr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Atlas with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = _serde::__private::Default::default();
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Atlas with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Atlas with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Atlas {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            atlas: __field3,
                            filename: __field4,
                            data: __field5,
                            data_size: __field6,
                            loaded: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<StringPtr> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "filename",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<StringPtr>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data_size",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("loaded"),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("filename")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("data_size")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("loaded")?
                            }
                        };
                        _serde::__private::Ok(Atlas {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            atlas: _serde::__private::Default::default(),
                            filename: __field4,
                            data: _serde::__private::Default::default(),
                            data_size: __field6,
                            loaded: __field7,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "filename",
                    "data_size",
                    "loaded",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Atlas",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Atlas>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Atlas {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                atlas: Pointer::default(),
                filename: StringPtr::default(),
                data: Pointer::default(),
                data_size: i32::default(),
                loaded: bool::default(),
            }
        }
    }
    impl Atlas {
        pub fn get_atlas(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_atlas(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_filename(&self) -> &str {
            unsafe {
                let member_ptr = toxoid_component_get_member_ptr(self.ptr, 8u32);
                let c_str: &core::ffi::CStr = unsafe {
                    core::ffi::CStr::from_ptr(member_ptr as *const i8)
                };
                let string_ptr: &str = c_str.to_str().unwrap();
                string_ptr
            }
        }
        pub fn set_filename(&mut self, value: StringPtr) {
            unsafe {
                toxoid_component_set_member_ptr(
                    self.ptr,
                    8u32,
                    value.ptr as *mut core::ffi::c_void,
                );
            }
        }
        pub fn get_data(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 16u32),
                }
            }
        }
        pub fn set_data(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 16u32, value.ptr);
            }
        }
        pub fn get_data_size(&self) -> i32 {
            unsafe { toxoid_component_get_member_i32(self.ptr, 24u32) }
        }
        pub fn set_data_size(&mut self, value: i32) {
            unsafe {
                toxoid_component_set_member_i32(self.ptr, 24u32, value);
            }
        }
        pub fn get_loaded(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 28u32) }
        }
        pub fn set_loaded(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 28u32, value);
            }
        }
    }
    impl ComponentType for Atlas {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Atlas",
                    &["atlas", "filename", "data", "data_size", "loaded"],
                    &[14u8, 11u8, 14u8, 6u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(5629617804758243756u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            5629617804758243756u64
        }
        fn get_name() -> &'static str {
            "Atlas"
        }
    }
    impl Component for Atlas {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Atlas"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Skeleton {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub skeleton: Pointer,
        pub filename: StringPtr,
        #[serde(skip)]
        pub data: Pointer,
        pub data_size: i32,
        pub loaded: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Skeleton {
        #[inline]
        fn clone(&self) -> Skeleton {
            Skeleton {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                skeleton: ::core::clone::Clone::clone(&self.skeleton),
                filename: ::core::clone::Clone::clone(&self.filename),
                data: ::core::clone::Clone::clone(&self.data),
                data_size: ::core::clone::Clone::clone(&self.data_size),
                loaded: ::core::clone::Clone::clone(&self.loaded),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Skeleton {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Skeleton {
        #[inline]
        fn eq(&self, other: &Skeleton) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.skeleton == other.skeleton
                && self.filename == other.filename && self.data == other.data
                && self.data_size == other.data_size && self.loaded == other.loaded
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Skeleton {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Skeleton",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "filename",
                    &self.filename,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data_size",
                    &self.data_size,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "loaded",
                    &self.loaded,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Skeleton {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __field6,
                    __field7,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            3u64 => _serde::__private::Ok(__Field::__field6),
                            4u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "filename" => _serde::__private::Ok(__Field::__field4),
                            "data_size" => _serde::__private::Ok(__Field::__field6),
                            "loaded" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"filename" => _serde::__private::Ok(__Field::__field4),
                            b"data_size" => _serde::__private::Ok(__Field::__field6),
                            b"loaded" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Skeleton>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Skeleton;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Skeleton",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Skeleton with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Skeleton with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            StringPtr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Skeleton with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = _serde::__private::Default::default();
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Skeleton with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Skeleton with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Skeleton {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            skeleton: __field3,
                            filename: __field4,
                            data: __field5,
                            data_size: __field6,
                            loaded: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<StringPtr> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "filename",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<StringPtr>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data_size",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("loaded"),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("filename")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("data_size")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("loaded")?
                            }
                        };
                        _serde::__private::Ok(Skeleton {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            skeleton: _serde::__private::Default::default(),
                            filename: __field4,
                            data: _serde::__private::Default::default(),
                            data_size: __field6,
                            loaded: __field7,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "filename",
                    "data_size",
                    "loaded",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Skeleton",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Skeleton>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Skeleton {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                skeleton: Pointer::default(),
                filename: StringPtr::default(),
                data: Pointer::default(),
                data_size: i32::default(),
                loaded: bool::default(),
            }
        }
    }
    impl Skeleton {
        pub fn get_skeleton(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_skeleton(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_filename(&self) -> &str {
            unsafe {
                let member_ptr = toxoid_component_get_member_ptr(self.ptr, 8u32);
                let c_str: &core::ffi::CStr = unsafe {
                    core::ffi::CStr::from_ptr(member_ptr as *const i8)
                };
                let string_ptr: &str = c_str.to_str().unwrap();
                string_ptr
            }
        }
        pub fn set_filename(&mut self, value: StringPtr) {
            unsafe {
                toxoid_component_set_member_ptr(
                    self.ptr,
                    8u32,
                    value.ptr as *mut core::ffi::c_void,
                );
            }
        }
        pub fn get_data(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 16u32),
                }
            }
        }
        pub fn set_data(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 16u32, value.ptr);
            }
        }
        pub fn get_data_size(&self) -> i32 {
            unsafe { toxoid_component_get_member_i32(self.ptr, 24u32) }
        }
        pub fn set_data_size(&mut self, value: i32) {
            unsafe {
                toxoid_component_set_member_i32(self.ptr, 24u32, value);
            }
        }
        pub fn get_loaded(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 28u32) }
        }
        pub fn set_loaded(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 28u32, value);
            }
        }
    }
    impl ComponentType for Skeleton {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Skeleton",
                    &["skeleton", "filename", "data", "data_size", "loaded"],
                    &[14u8, 11u8, 14u8, 6u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(8342546149565225806u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            8342546149565225806u64
        }
        fn get_name() -> &'static str {
            "Skeleton"
        }
    }
    impl Component for Skeleton {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Skeleton"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Images {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub images: Pointer,
        pub loaded: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Images {
        #[inline]
        fn clone(&self) -> Images {
            Images {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                images: ::core::clone::Clone::clone(&self.images),
                loaded: ::core::clone::Clone::clone(&self.loaded),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Images {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Images {
        #[inline]
        fn eq(&self, other: &Images) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.images == other.images
                && self.loaded == other.loaded
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Images {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Images",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "loaded",
                    &self.loaded,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Images {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "loaded" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"loaded" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Images>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Images;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Images",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Images with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Images with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Images with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Images {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            images: __field3,
                            loaded: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("loaded"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("loaded")?
                            }
                        };
                        _serde::__private::Ok(Images {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            images: _serde::__private::Default::default(),
                            loaded: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id", "loaded"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Images",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Images>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Images {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                images: Pointer::default(),
                loaded: bool::default(),
            }
        }
    }
    impl Images {
        pub fn get_images(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_images(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_loaded(&self) -> bool {
            unsafe { toxoid_component_get_member_bool(self.ptr, 8u32) }
        }
        pub fn set_loaded(&mut self, value: bool) {
            unsafe {
                toxoid_component_set_member_bool(self.ptr, 8u32, value);
            }
        }
    }
    impl ComponentType for Images {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "Images",
                    &["images", "loaded"],
                    &[14u8, 10u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(18196004514279928267u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            18196004514279928267u64
        }
        fn get_name() -> &'static str {
            "Images"
        }
    }
    impl Component for Images {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Images"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct BoneAnimation {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        pub animation_state: StringPtr,
        pub animation: StringPtr,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BoneAnimation {
        #[inline]
        fn clone(&self) -> BoneAnimation {
            BoneAnimation {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                animation_state: ::core::clone::Clone::clone(&self.animation_state),
                animation: ::core::clone::Clone::clone(&self.animation),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BoneAnimation {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BoneAnimation {
        #[inline]
        fn eq(&self, other: &BoneAnimation) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.animation_state == other.animation_state
                && self.animation == other.animation
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BoneAnimation {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "BoneAnimation",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "animation_state",
                    &self.animation_state,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "animation",
                    &self.animation,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BoneAnimation {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            3u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "animation_state" => _serde::__private::Ok(__Field::__field3),
                            "animation" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"animation_state" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"animation" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<BoneAnimation>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BoneAnimation;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct BoneAnimation",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct BoneAnimation with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct BoneAnimation with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            StringPtr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct BoneAnimation with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            StringPtr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct BoneAnimation with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(BoneAnimation {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            animation_state: __field3,
                            animation: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<StringPtr> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<StringPtr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "animation_state",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<StringPtr>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "animation",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<StringPtr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("animation_state")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("animation")?
                            }
                        };
                        _serde::__private::Ok(BoneAnimation {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            animation_state: __field3,
                            animation: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "singleton",
                    "id",
                    "animation_state",
                    "animation",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BoneAnimation",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<BoneAnimation>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for BoneAnimation {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                animation_state: StringPtr::default(),
                animation: StringPtr::default(),
            }
        }
    }
    impl BoneAnimation {
        pub fn get_animation_state(&self) -> &str {
            unsafe {
                let member_ptr = toxoid_component_get_member_ptr(self.ptr, 0u32);
                let c_str: &core::ffi::CStr = unsafe {
                    core::ffi::CStr::from_ptr(member_ptr as *const i8)
                };
                let string_ptr: &str = c_str.to_str().unwrap();
                string_ptr
            }
        }
        pub fn set_animation_state(&mut self, value: StringPtr) {
            unsafe {
                toxoid_component_set_member_ptr(
                    self.ptr,
                    0u32,
                    value.ptr as *mut core::ffi::c_void,
                );
            }
        }
        pub fn get_animation(&self) -> &str {
            unsafe {
                let member_ptr = toxoid_component_get_member_ptr(self.ptr, 8u32);
                let c_str: &core::ffi::CStr = unsafe {
                    core::ffi::CStr::from_ptr(member_ptr as *const i8)
                };
                let string_ptr: &str = c_str.to_str().unwrap();
                string_ptr
            }
        }
        pub fn set_animation(&mut self, value: StringPtr) {
            unsafe {
                toxoid_component_set_member_ptr(
                    self.ptr,
                    8u32,
                    value.ptr as *mut core::ffi::c_void,
                );
            }
        }
    }
    impl ComponentType for BoneAnimation {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "BoneAnimation",
                    &["animation_state", "animation"],
                    &[11u8, 11u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(9574222768534778823u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            9574222768534778823u64
        }
        fn get_name() -> &'static str {
            "BoneAnimation"
        }
    }
    impl Component for BoneAnimation {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("BoneAnimation"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct TiledWorldComponent {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub world: Pointer,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TiledWorldComponent {
        #[inline]
        fn clone(&self) -> TiledWorldComponent {
            TiledWorldComponent {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                world: ::core::clone::Clone::clone(&self.world),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TiledWorldComponent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TiledWorldComponent {
        #[inline]
        fn eq(&self, other: &TiledWorldComponent) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.world == other.world
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TiledWorldComponent {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TiledWorldComponent",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TiledWorldComponent {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TiledWorldComponent>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TiledWorldComponent;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TiledWorldComponent",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TiledWorldComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TiledWorldComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        _serde::__private::Ok(TiledWorldComponent {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            world: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(TiledWorldComponent {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            world: _serde::__private::Default::default(),
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TiledWorldComponent",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TiledWorldComponent>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for TiledWorldComponent {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                world: Pointer::default(),
            }
        }
    }
    impl TiledWorldComponent {
        pub fn get_world(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_world(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
    }
    impl ComponentType for TiledWorldComponent {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("TiledWorldComponent", &["world"], &[14u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(12854511656334294528u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            12854511656334294528u64
        }
        fn get_name() -> &'static str {
            "TiledWorldComponent"
        }
    }
    impl Component for TiledWorldComponent {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("TiledWorldComponent"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct TiledCellComponent {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub cell: Pointer,
        pub index: u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TiledCellComponent {
        #[inline]
        fn clone(&self) -> TiledCellComponent {
            TiledCellComponent {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                cell: ::core::clone::Clone::clone(&self.cell),
                index: ::core::clone::Clone::clone(&self.index),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TiledCellComponent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TiledCellComponent {
        #[inline]
        fn eq(&self, other: &TiledCellComponent) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.cell == other.cell
                && self.index == other.index
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TiledCellComponent {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TiledCellComponent",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "index",
                    &self.index,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TiledCellComponent {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            "index" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            b"index" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TiledCellComponent>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TiledCellComponent;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TiledCellComponent",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TiledCellComponent with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TiledCellComponent with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct TiledCellComponent with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TiledCellComponent {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            cell: __field3,
                            index: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("index")?
                            }
                        };
                        _serde::__private::Ok(TiledCellComponent {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            cell: _serde::__private::Default::default(),
                            index: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id", "index"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TiledCellComponent",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TiledCellComponent>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for TiledCellComponent {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                cell: Pointer::default(),
                index: u32::default(),
            }
        }
    }
    impl TiledCellComponent {
        pub fn get_cell(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_cell(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
        pub fn get_index(&self) -> u32 {
            unsafe { toxoid_component_get_member_u32(self.ptr, 8u32) }
        }
        pub fn set_index(&mut self, value: u32) {
            unsafe {
                toxoid_component_set_member_u32(self.ptr, 8u32, value);
            }
        }
    }
    impl ComponentType for TiledCellComponent {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs(
                    "TiledCellComponent",
                    &["cell", "index"],
                    &[14u8, 2u8],
                )
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(10731055677508222166u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            10731055677508222166u64
        }
        fn get_name() -> &'static str {
            "TiledCellComponent"
        }
    }
    impl Component for TiledCellComponent {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("TiledCellComponent"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct TilesetComponent {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TilesetComponent {
        #[inline]
        fn clone(&self) -> TilesetComponent {
            TilesetComponent {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TilesetComponent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TilesetComponent {
        #[inline]
        fn eq(&self, other: &TilesetComponent) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TilesetComponent {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TilesetComponent",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TilesetComponent {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TilesetComponent>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TilesetComponent;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TilesetComponent",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TilesetComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TilesetComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TilesetComponent {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(TilesetComponent {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TilesetComponent",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TilesetComponent>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for TilesetComponent {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl TilesetComponent {}
    impl ComponentType for TilesetComponent {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("TilesetComponent", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(15542299914431747514u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            15542299914431747514u64
        }
        fn get_name() -> &'static str {
            "TilesetComponent"
        }
    }
    impl Component for TilesetComponent {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("TilesetComponent"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Callback {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
        #[serde(skip)]
        pub callback: Pointer,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Callback {
        #[inline]
        fn clone(&self) -> Callback {
            Callback {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
                callback: ::core::clone::Clone::clone(&self.callback),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Callback {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Callback {
        #[inline]
        fn eq(&self, other: &Callback) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id && self.callback == other.callback
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Callback {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Callback",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Callback {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Callback>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Callback;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Callback",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Callback with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Callback with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = _serde::__private::Default::default();
                        _serde::__private::Ok(Callback {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                            callback: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Callback {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                            callback: _serde::__private::Default::default(),
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Callback",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Callback>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Callback {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
                callback: Pointer::default(),
            }
        }
    }
    impl Callback {
        pub fn get_callback(&self) -> Pointer {
            unsafe {
                Pointer {
                    ptr: toxoid_component_get_member_ptr(self.ptr, 0u32),
                }
            }
        }
        pub fn set_callback(&mut self, value: Pointer) {
            unsafe {
                toxoid_component_set_member_ptr(self.ptr, 0u32, value.ptr);
            }
        }
    }
    impl ComponentType for Callback {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Callback", &["callback"], &[14u8])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(16213323499742365414u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            16213323499742365414u64
        }
        fn get_name() -> &'static str {
            "Callback"
        }
    }
    impl Component for Callback {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Callback"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Rect {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Rect {
        #[inline]
        fn clone(&self) -> Rect {
            Rect {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Rect {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Rect {
        #[inline]
        fn eq(&self, other: &Rect) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Rect {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Rect",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Rect {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Rect>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Rect;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Rect",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Rect with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Rect with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Rect {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Rect {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Rect",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Rect>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Rect {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Rect {}
    impl ComponentType for Rect {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Rect", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(8364433741114752039u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            8364433741114752039u64
        }
        fn get_name() -> &'static str {
            "Rect"
        }
    }
    impl Component for Rect {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe { toxoid_component_lookup(toxoid_make_c_string("Rect")) })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Local {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Local {
        #[inline]
        fn clone(&self) -> Local {
            Local {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Local {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Local {
        #[inline]
        fn eq(&self, other: &Local) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Local {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Local",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Local {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Local>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Local;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Local",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Local with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Local with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Local {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Local {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Local",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Local>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Local {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Local {}
    impl ComponentType for Local {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Local", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(16839520613172088648u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            16839520613172088648u64
        }
        fn get_name() -> &'static str {
            "Local"
        }
    }
    impl Component for Local {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Local"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Remote {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Remote {
        #[inline]
        fn clone(&self) -> Remote {
            Remote {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Remote {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Remote {
        #[inline]
        fn eq(&self, other: &Remote) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Remote {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Remote",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Remote {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Remote>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Remote;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Remote",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Remote with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Remote with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Remote {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Remote {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Remote",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Remote>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Remote {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Remote {}
    impl ComponentType for Remote {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Remote", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(16881730082269474995u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            16881730082269474995u64
        }
        fn get_name() -> &'static str {
            "Remote"
        }
    }
    impl Component for Remote {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Remote"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Player {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Player {
        #[inline]
        fn clone(&self) -> Player {
            Player {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Player {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Player {
        #[inline]
        fn eq(&self, other: &Player) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Player {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Player",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Player {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Player>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Player;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Player",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Player with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Player with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Player {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Player {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Player",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Player>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Player {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Player {}
    impl ComponentType for Player {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Player", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(3692324345213718176u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            3692324345213718176u64
        }
        fn get_name() -> &'static str {
            "Player"
        }
    }
    impl Component for Player {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Player"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Light {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Light {
        #[inline]
        fn clone(&self) -> Light {
            Light {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Light {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Light {
        #[inline]
        fn eq(&self, other: &Light) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Light {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Light",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Light {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Light>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Light;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Light",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Light with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Light with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Light {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Light {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Light",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Light>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Light {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Light {}
    impl ComponentType for Light {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Light", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(4254411488076640623u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            4254411488076640623u64
        }
        fn get_name() -> &'static str {
            "Light"
        }
    }
    impl Component for Light {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Light"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Loadable {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Loadable {
        #[inline]
        fn clone(&self) -> Loadable {
            Loadable {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Loadable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Loadable {
        #[inline]
        fn eq(&self, other: &Loadable) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Loadable {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Loadable",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Loadable {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Loadable>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Loadable;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Loadable",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Loadable with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Loadable with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Loadable {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Loadable {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Loadable",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Loadable>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Loadable {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Loadable {}
    impl ComponentType for Loadable {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Loadable", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(533396289736451033u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            533396289736451033u64
        }
        fn get_name() -> &'static str {
            "Loadable"
        }
    }
    impl Component for Loadable {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Loadable"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Blittable {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Blittable {
        #[inline]
        fn clone(&self) -> Blittable {
            Blittable {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Blittable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Blittable {
        #[inline]
        fn eq(&self, other: &Blittable) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Blittable {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Blittable",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Blittable {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Blittable>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Blittable;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Blittable",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Blittable with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Blittable with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Blittable {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Blittable {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Blittable",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Blittable>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Blittable {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Blittable {}
    impl ComponentType for Blittable {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Blittable", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(7300871801761279178u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            7300871801761279178u64
        }
        fn get_name() -> &'static str {
            "Blittable"
        }
    }
    impl Component for Blittable {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Blittable"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Renderable {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Renderable {
        #[inline]
        fn clone(&self) -> Renderable {
            Renderable {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Renderable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Renderable {
        #[inline]
        fn eq(&self, other: &Renderable) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Renderable {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Renderable",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Renderable {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Renderable>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Renderable;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Renderable",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Renderable with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Renderable with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Renderable {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Renderable {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Renderable",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Renderable>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Renderable {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Renderable {}
    impl ComponentType for Renderable {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Renderable", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(5334664629576072941u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            5334664629576072941u64
        }
        fn get_name() -> &'static str {
            "Renderable"
        }
    }
    impl Component for Renderable {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Renderable"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Connected {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Connected {
        #[inline]
        fn clone(&self) -> Connected {
            Connected {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Connected {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Connected {
        #[inline]
        fn eq(&self, other: &Connected) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Connected {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Connected",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Connected {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Connected>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Connected;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Connected",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Connected with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Connected with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Connected {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Connected {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Connected",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Connected>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Connected {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Connected {}
    impl ComponentType for Connected {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Connected", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(9413898206493107824u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            9413898206493107824u64
        }
        fn get_name() -> &'static str {
            "Connected"
        }
    }
    impl Component for Connected {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Connected"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    #[repr(C)]
    pub struct Disconnected {
        #[serde(skip, default = "default_ptr")]
        ptr: *mut core::ffi::c_void,
        singleton: bool,
        id: ecs_entity_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Disconnected {
        #[inline]
        fn clone(&self) -> Disconnected {
            Disconnected {
                ptr: ::core::clone::Clone::clone(&self.ptr),
                singleton: ::core::clone::Clone::clone(&self.singleton),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Disconnected {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Disconnected {
        #[inline]
        fn eq(&self, other: &Disconnected) -> bool {
            self.ptr == other.ptr && self.singleton == other.singleton
                && self.id == other.id
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Disconnected {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Disconnected",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singleton",
                    &self.singleton,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Disconnected {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singleton" => _serde::__private::Ok(__Field::__field1),
                            "id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singleton" => _serde::__private::Ok(__Field::__field1),
                            b"id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Disconnected>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Disconnected;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Disconnected",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = default_ptr();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Disconnected with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            ecs_entity_t,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Disconnected with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Disconnected {
                            ptr: __field0,
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<ecs_entity_t> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singleton",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ecs_entity_t,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("singleton")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Disconnected {
                            ptr: default_ptr(),
                            singleton: __field1,
                            id: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singleton", "id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Disconnected",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Disconnected>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for Disconnected {
        fn default() -> Self {
            Self {
                ptr: ::std::ptr::null_mut(),
                singleton: false,
                id: 0,
            }
        }
    }
    impl Disconnected {}
    impl ComponentType for Disconnected {
        fn register() -> u64 {
            let component_id_split = unsafe {
                toxoid_register_component_ecs("Disconnected", &[], &[])
            };
            let component_id = combine_u32(component_id_split);
            let type_hash = split_u64(18170964375127462538u64);
            cache_component_ecs(type_hash, split_u64(component_id));
            component_id
        }
        fn get_hash() -> u64 {
            18170964375127462538u64
        }
        fn get_name() -> &'static str {
            "Disconnected"
        }
    }
    impl Component for Disconnected {
        fn get_id(&self) -> u64 {
            combine_u32(unsafe {
                toxoid_component_lookup(toxoid_make_c_string("Disconnected"))
            })
        }
        fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
            self.ptr = ptr;
        }
        fn get_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
        }
        fn set_singleton(&mut self, singleton: bool) {
            self.singleton = singleton;
        }
        fn get_singleton(&self) -> bool {
            self.singleton
        }
    }
    pub fn init() {
        GameConfig::register();
        KeyboardInput::register();
        WebSocket::register();
        Position::register();
        Velocity::register();
        Size::register();
        Color::register();
        Sprite::register();
        BlendMode::register();
        RenderTarget::register();
        Networked::register();
        Atlas::register();
        Skeleton::register();
        Images::register();
        BoneAnimation::register();
        Direction::register();
        TiledWorldComponent::register();
        TiledCellComponent::register();
        TilesetComponent::register();
        SpineInstance::register();
        Callback::register();
        Rect::register();
        Loadable::register();
        Blittable::register();
        Renderable::register();
        Connected::register();
        Disconnected::register();
        Local::register();
        Remote::register();
        Player::register();
        Light::register();
    }
}
pub mod utils {
    use crate::*;
    const FNV_PRIME: u64 = 1099511628211;
    const OFFSET_BASIS: u64 = 14695981039346656037;
    pub fn fnv1a_hash(bytes: &[u8]) -> u64 {
        bytes
            .iter()
            .fold(
                OFFSET_BASIS,
                |hash, &byte| { (hash ^ byte as u64).wrapping_mul(FNV_PRIME) },
            )
    }
    pub fn get_timestamp() -> f64 {
        unsafe { combine_f32(toxoid_get_timestamp()) }
    }
    pub fn make_c_string(string: &str) -> *mut i8 {
        unsafe { toxoid_make_c_string(string) }
    }
}
pub mod net {
    use crate::*;
    use serde::{Deserialize, Serialize};
    #[repr(C)]
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub struct MessageComponent {
        pub name: String,
        pub data: Vec<u8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MessageComponent {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MessageComponent",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data",
                    &self.data,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MessageComponent {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "data" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"data" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MessageComponent>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MessageComponent;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MessageComponent",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MessageComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<u8>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct MessageComponent with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MessageComponent {
                            name: __field0,
                            data: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec<u8>>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("data")?
                            }
                        };
                        _serde::__private::Ok(MessageComponent {
                            name: __field0,
                            data: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name", "data"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MessageComponent",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MessageComponent>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[repr(C)]
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub struct MessageEntity {
        pub id: u64,
        pub event: String,
        pub components: Vec<MessageComponent>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MessageEntity {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MessageEntity",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "event",
                    &self.event,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "components",
                    &self.components,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MessageEntity {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "event" => _serde::__private::Ok(__Field::__field1),
                            "components" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"event" => _serde::__private::Ok(__Field::__field1),
                            b"components" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MessageEntity>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MessageEntity;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MessageEntity",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MessageEntity with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct MessageEntity with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Vec<MessageComponent>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct MessageEntity with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MessageEntity {
                            id: __field0,
                            event: __field1,
                            components: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Vec<MessageComponent>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("event"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "components",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<MessageComponent>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("event")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("components")?
                            }
                        };
                        _serde::__private::Ok(MessageEntity {
                            id: __field0,
                            event: __field1,
                            components: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["id", "event", "components"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MessageEntity",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MessageEntity>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[repr(C)]
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub struct Messages {
        pub messages: Vec<MessageEntity>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Messages {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Messages",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "messages",
                    &self.messages,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Messages {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "messages" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"messages" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Messages>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Messages;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Messages",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<MessageEntity>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Messages with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Messages { messages: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Vec<MessageEntity>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "messages",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<MessageEntity>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("messages")?
                            }
                        };
                        _serde::__private::Ok(Messages { messages: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["messages"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Messages",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Messages>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub fn send_components(
        entity: &mut Entity,
        components: &[&dyn Component],
        event: &str,
    ) {
        unsafe {
            toxoid_net_send_components(split_u64(entity.get_id()), components, event);
        }
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn network_entity_cache_insert(local_id: u64, network_id: u64) {
        unsafe { toxoid_network_entity_cache_insert(local_id, network_id) }
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn network_entity_cache_get(local_id: u64) -> u64 {
        unsafe { toxoid_network_entity_cache_get(local_id) }
    }
    #[cfg(
        any(
            not(target_arch = "wasm32"),
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )]
    pub fn network_entity_cache_remove(local_id: u64) {
        unsafe { toxoid_network_entity_cache_remove(local_id) }
    }
    pub fn deserialize_entity_sync(
        entity_id: ecs_entity_t,
        components_serialized: &[crate::net::MessageComponent],
    ) {
        unsafe { toxoid_deserialize_entity_sync(entity_id, components_serialized) }
    }
}
pub mod load {
    use crate::*;
    pub fn load_sprite(callback: impl FnMut(&mut Entity) + 'static) -> Entity {
        Entity::new()
    }
    pub fn load_worldmap() -> Entity {
        Entity::new()
    }
    pub fn load_animation() -> Entity {
        Entity::new()
    }
}
pub mod events {
    use crate::*;
    pub fn init() {}
}
pub mod bootstrap {
    use crate::*;
    pub fn bootstrap_game_config() {
        World::add_singleton::<GameConfig>();
        let mut game_config = World::get_singleton::<GameConfig>();
        game_config.set_resolution_width(1280);
        game_config.set_resolution_height(720);
    }
    pub fn bootstrap_input() {
        World::add_singleton::<KeyboardInput>();
    }
    #[cfg(feature = "net")]
    pub fn bootstrap_network() {
        World::add_singleton::<Networked>();
    }
    pub fn bootstrap_player() {
        #[cfg(feature = "net")]
        let mut networked = World::get_singleton::<Networked>();
        let mut player_entity = Entity::new();
        player_entity.add::<Player>();
        player_entity.add::<Local>();
        #[cfg(feature = "net")] player_entity.add::<Networked>();
        #[cfg(feature = "net")] networked.set_entity_id(player_entity.get_id());
    }
    pub fn default() {
        bootstrap_game_config();
        bootstrap_input();
        #[cfg(feature = "net")] bootstrap_network();
        bootstrap_player();
    }
}
pub use bindings::*;
pub use ecs::*;
pub use globals::*;
pub use log::*;
pub use components::*;
pub use serde;
pub use utils::*;
pub use core::ffi::c_void;
pub use net::*;
pub use load::*;
pub fn init() {
    events::init();
}
