use toxoid_api::*;
use crate::entities::*;
use crate::components::*;

// Direction enum
enum DirectionEnum {
    Up,
    Down,
    Left,
    Right
}

#[components(Position, Head)]
fn test_system(iter: &Iter) {
    for (pos, _head) in components {
        println!("Position: {:?}", pos.get_x());
    }
}

pub fn init() {    
    // Movement System
    System::dsl("Head, Position", Some(10), |iter| {
        iter.entities().iter_mut().for_each(|snake_entity| {
            // println!("Snake entity id: {:?}", snake_entity.get_id());
            // println!("Snake entity children: {:?}", snake_entity.children().len());
            // Keeping track of all tail entities
            let mut tails = World::get_singleton::<Tails>();

            // Get current position of head
            let mut pos = snake_entity.get::<Position>();
            let size = snake_entity.get::<Size>();
            let current_x = pos.get_x();
            let current_y = pos.get_y();

            // Get current position of food
            let food_singleton = World::get_singleton::<FoodEntity>();
            let food_entity_id = food_singleton.get_entity();
            let mut food_entity = World::get_entity(food_entity_id);
            let food_pos = food_entity.get::<Position>();
            let food_size = food_entity.get::<Size>();

            // Check if head and food are colliding
            if aabb(&pos, &size, &food_pos, &food_size) {
                // Set (respawning) random position for food   
                let grid_size = 50;
                let mut food_pos = food_entity.get::<Position>();
                food_pos.set_x(get_random((SCREEN_WIDTH - 100) / grid_size) * grid_size);
                food_pos.set_y(get_random((SCREEN_HEIGHT - 100) / grid_size) * grid_size);
                // Increase the tail length
                tails.set_max_length(tails.get_max_length() + 1);
            }

            // // Create new head entity on every movement tick
            // let mut new_snake_head_entity = entities::create_snake_head();
            // let mut pos = new_snake_head_entity.get::<Position>();
            // new_snake_head_entity.add::<Head>();
            // new_snake_head_entity.remove::<Tail>();
            // pos.set_x(current_x);
            // pos.set_y(current_y);
            // new_snake_head_entity.set_name("SnakeEntityHead".to_string());
            
            // // Remove head tag from old head entity
            // snake_entity.add::<Tail>();
            // snake_entity.remove::<Head>();
            // snake_entity.child_of(new_snake_head_entity);
            // println!("Snake entity children: {:?}", snake_entity.children().len());
            // snake_entity.set_name(format!("SnakeEntityTail {}", tails.get_max_length()));

            // Movement logic
            let direction = World::get_singleton::<Direction>();
            let screen_y_bounds = SCREEN_HEIGHT - 100;
            let screen_x_bounds = SCREEN_WIDTH - 100;
            
            match direction.get_direction() {
                d if d == DirectionEnum::Up as u8 => if current_y >= 50 { pos.set_y(current_y - 50) },
                d if d == DirectionEnum::Down as u8 => if current_y <= screen_y_bounds { pos.set_y(current_y + 50) },
                d if d == DirectionEnum::Left as u8 => if current_x >= 50 { pos.set_x(current_x - 50) },
                d if d == DirectionEnum::Right as u8 => if current_x <= screen_x_bounds { pos.set_x(current_x + 50) },
                _ => {}
            }

            // Store previous head position before updating
            let prev_x = current_x;
            let prev_y = current_y;
            
            // TODO: Do this linked list style until we implement query sorting
            // snake_entity
            //     .children()
            //     .sort_by(|a, b| a.get::<Order>().get_index().cmp(&b.get::<Order>().get_index()));
            //     .iter_mut()
            //     .fold((prev_x, prev_y), |(prev_x, prev_y), child| {
            //         println!("Child id: {:?}", child.get_id());
            //         let mut child_pos = child.get::<Position>();
            //         let old_x = child_pos.get_x();
            //         let old_y = child_pos.get_y();
            //         child_pos.set_x(prev_x);
            //         child_pos.set_y(prev_y);
            //         (old_x, old_y)
            //     });

            // Recursively update children positions
            fn update_child_positions(entity: &mut Entity, prev_x: u32, prev_y: u32) -> (u32, u32) {
                let mut child_pos = entity.get::<Position>();
                let old_x = child_pos.get_x();
                let old_y = child_pos.get_y();
                child_pos.set_x(prev_x);
                child_pos.set_y(prev_y);

                // Update each child recursively
                for child in entity.children().iter_mut() {
                    update_child_positions(child, old_x, old_y);
                }
                
                (old_x, old_y)
            }

            // Update all immediate children with the head's previous position
            for child in snake_entity.children().iter_mut() {
                update_child_positions(child, prev_x, prev_y);
            }
        });
    })
        .build();

    // Input System
    System::dsl("KeyboardInput", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut direction = World::get_singleton::<Direction>();
            let mut keyboard_input = entity.get::<KeyboardInput>();
            if keyboard_input.get_up() {
                direction.set_direction(DirectionEnum::Up as u8);
                keyboard_input.set_up(false);
            }
            if keyboard_input.get_down() {
                direction.set_direction(DirectionEnum::Down as u8);
                keyboard_input.set_down(false);
            }
            if keyboard_input.get_left() {
                direction.set_direction(DirectionEnum::Left as u8);
                keyboard_input.set_left(false);
            }
            if keyboard_input.get_right() {
                direction.set_direction(DirectionEnum::Right as u8);
                keyboard_input.set_right(false);
            }
        });
    })
        .build();

    // Observer::dsl("Position", vec![Event::OnSet], |iter| {
    //     println!("Observer from guest called");
    //     iter.entities().iter_mut().for_each(|entity| {
    //         let pos = entity.get::<Position>();
    //         println!("Position: {:?}", pos.get_x());
    //     });
    // })
    //     .build();

    // System::dsl("", None, |_iter| {
    //     Query::dsl_each("Position", |query| {
    //         while query.next() {
    //             let positions = query.components::<Position>(0);
    //             positions.iter().for_each(|position| {
    //                 println!("Position X: {:?}, Y: {:?}", position.get_x(), position.get_y());
    //             });
    //         }
    //     });
    // })
    //     .build();

    // System::dsl("Position, Head", None, test_system)
    //     .build();
}