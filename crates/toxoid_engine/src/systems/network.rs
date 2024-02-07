// use toxoid_api::*;
// #[cfg(feature = "net")]
// use toxoid_net::NetworkMessages;

// #[cfg(feature = "net")]
// pub fn network_event_system(query: &mut Query) {
//     let query = query.iter();
//     while query.next() {
//         let entities = query.entities();
//         entities
//             .iter_mut()
//             .for_each(|entity| {
//                 let net = entity.get::<Networked>();
//                 let messages = net.get_messages().ptr as *mut NetworkMessages;
//                 unsafe {
//                     (*messages)
//                         .messages
//                         .iter()
//                         .for_each(|message| 
//                             toxoid_ffi::ecs::toxoid_run_network_event(message.event.as_str(), message)
//                         );
//                         // The drop function in Rust is used to free the memory that is owned by a value. However, raw pointers in Rust do not own the memory they point to. Instead, they are just a reference to a location in memory. Therefore, calling drop on a raw pointer does not deallocate the memory it points to.
//                         // Convert the raw pointer back into a Box using Box::from_raw. This will create a Box that takes ownership of the memory, and when the Box is dropped at the end of the scope, it will deallocate the memory.
//                         let _ = Box::from_raw(net.get_messages().ptr);
//                 }
//                 entity.remove::<Updated>();
//             });
//     }
//     // let network_messages = toxoid_net::deserialize(data);
//     // network_messages
//     //     .messages
//     //     .iter()
//     //     .for_each(|entity| {
//     //         // println!("Event received: {:?}", entity.event);
//     //         match entity.event.as_str() {
//     //             "LocalPlayerJoin" => {
//     //                 println!("Local player ID received: {:?}", entity.id);
//     //                 // Set local player ID
//     //                 let mut local_player = World::get_singleton::<Networked>();
//     //                 local_player.set_id(entity.id);

//     //                 // Create entity
//     //                 let render_entity = crate::utils::load::load_image("assets/character.png");
//     //                 render_entity.add::<Local>();

//     //                 // Add to network entity cache
//     //                 unsafe { toxoid_network_entity_cache_insert(split_u64(entity.id), split_u64(render_entity.get_id())) };
//     //             },
//     //             "PlayerJoin" => {
//     //                 println!("ID received: {:?}", entity.id);
//     //                 // Create entity
//     //                 let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
//     //                 let mut position = player_animation_entity.get::<Position>();
//     //                 position.set_x(100);
//     //                 position.set_y(100);
//     //                 player_animation_entity.add::<Remote>();
                    
//     //                 // Update position
//     //                 let deserialized_component = entity.components[0].clone();
//     //                 let mut position = player_animation_entity.get::<Position>();
//     //                 // position.set_x(deserialized_component.x);
//     //                 // position.set_y(deserialized_component.y);
                    
//     //                 // Add to network entity cache
//     //                 unsafe { toxoid_network_entity_cache_insert(split_u64(entity.id), split_u64(player_animation_entity.get_id())) };
//     //             },
//     //             "PlayerLeave" => {
//     //                 println!("Player ID {:?} disconnected from server.", entity.id);
//     //             },
//     //             "PlayerMove" => {
//     //                 // println!("Player ID {:?} moved to position {:?}.", entity.id, entity.components[0]);
//     //                 // Get entity from network entity cache
//     //                 let entity_id = unsafe { toxoid_network_entity_cache_get(split_u64(entity.id)) };
//     //                 let entity_id = combine_u32(entity_id);
//     //                 if entity_id == 0 {
//     //                     return;
//     //                 }

//     //                 // Create entity object
//     //                 let render_entity = Entity {
//     //                     id: entity_id,
//     //                     children: &mut []
//     //                 };
                    
//     //                 // Update position
//     //                 let mut position = render_entity.get::<Position>();
//     //                 // position.set_x(entity.components[0].x);
//     //                 // position.set_y(entity.components[0].y);
//     //             },
//     //             _ => {}
//     //         }
//     //     });
// }