use toxoid_api::*;
use crate::components::*;
use rand::{Rng, thread_rng};

// TODO: Move this config ECS singleton
pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
const DEFAULT_TAIL_LENGTH: u32 = 2;

pub fn get_random(max: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

pub fn aabb(a: &Position, a2: &Size, b: &Position, b2: &Size) -> bool {
    a.get_x() + a2.get_width() > b.get_x() &&
    a.get_x() < b.get_x() + b2.get_width() &&
    a.get_y() + a2.get_height() > b.get_y() &&
    a.get_y() < b.get_y() + b2.get_height()
}

pub fn create_snake_head() -> Entity {
    let exists = World::has_entity_named("SnakeEntityHead".to_string());
    let mut head_entity = Entity::new(None);
    head_entity.add::<Head>();
    head_entity.add::<Position>();
    head_entity.add::<Size>();
    head_entity.add::<Color>();
    head_entity.add::<Renderable>();
    head_entity.add::<Rect>();

    if !exists {
        let mut pos = head_entity.get::<Position>();
        pos.set_x((SCREEN_WIDTH - 100) / 2);
        pos.set_y((SCREEN_HEIGHT - 100) / 2);
    }

    let mut size = head_entity.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = head_entity.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);

    head_entity
}

pub fn create_snake_tail(tail_num: u32) -> Entity {
    let exists = World::has_entity_named(format!("SnakeEntityTail {tail_num}"));
    let mut tail_entity = Entity::new(None);
    tail_entity.add::<Tail>();
    tail_entity.add::<Position>();
    tail_entity.add::<Size>();
    tail_entity.add::<Color>();
    tail_entity.add::<Renderable>();
    tail_entity.add::<Rect>();

    if !exists {
        let mut pos = tail_entity.get::<Position>();
        pos.set_x((SCREEN_WIDTH - 100) / 2);
        pos.set_y(((SCREEN_HEIGHT - 100) / 2) - (tail_num * 50));
    }

    let mut size = tail_entity.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = tail_entity.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);
    
    tail_entity
}

pub fn create_food() {
    // Check if the entity exists for hot reloading conditionals
    // let exists = World::has_entity_named("FoodNamedEntity".to_string());

    let mut food = Entity::prefab_named("FoodNamedEntityPrefab");
    food.add::<Food>();
    food.add::<Position>();
    food.add::<Size>();
    food.add::<Color>();
    food.add::<Renderable>();
    food.add::<Rect>();

    // If hot reloading, don't change the position on initialization
    // if !exists {
        let mut pos = food.get::<Position>();
        let grid_size = 50;
        pos.set_x(get_random((SCREEN_WIDTH - 100) / grid_size) * grid_size);
        pos.set_y(get_random((SCREEN_HEIGHT - 100) / grid_size) * grid_size);
    // }

    let mut size = food.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = food.get::<Color>();
    color.set_r(1.);
    color.set_g(0.);
    color.set_b(0.);
    color.set_a(1.);

    let mut food_instance = Entity::from_prefab_id(None, food.get_id());
    println!("Food instance 1 has size: {}", food_instance.has::<Size>());
    println!("Food instance 1 has food: {}", food_instance.has::<Food>());
    println!("Food instance 1 has position: {}", food_instance.has::<Position>());
    println!("Food instance 1 has rect: {}", food_instance.has::<Rect>());
    println!("Food instance 1 has snake head: {}", food_instance.has::<Head>());
    println!("Food instance 1, Position X: {}", food_instance.get::<Position>().get_x());
    println!("Food instance 1, Position Y: {}", food_instance.get::<Position>().get_y());
    

    let mut food_entity = World::get_singleton::<FoodEntity>();
    food_entity.set_entity(food_instance.get_id());

    let mut food_instance_2 = Entity::from_prefab_id(None, food.get_id());
    println!("Food instance 2 has size: {}", food_instance_2.has::<Size>());
    println!("Food instance 2 has food: {}", food_instance_2.has::<Food>());
    println!("Food instance 2 has position: {}", food_instance_2.has::<Position>());
    println!("Food instance 2 has rect: {}", food_instance_2.has::<Rect>());
    println!("Food instance 2 has snake head: {}", food_instance_2.has::<Head>());
    let mut pos = food_instance_2.get::<Position>();
    pos.set_x(777);
    pos.set_y(777);
    println!("Food instance 2, Position X: {}", pos.get_x());
    println!("Food instance 2, Position Y: {}", pos.get_y());

    println!("Food instance 1, Position X: {}", food_instance.get::<Position>().get_x());
    println!("Food instance 1, Position Y: {}", food_instance.get::<Position>().get_y());
    
    let mut food_entity = World::get_singleton::<FoodEntity>();
    food_entity.set_entity(food_instance.get_id());
}

pub fn init() {
    // Initialize Tails singleton
    let mut tails = World::get_singleton::<Tails>();
    tails.set_max_length(DEFAULT_TAIL_LENGTH);
    if !World::has_entity_named("CreateSnakeOnce".to_string()) {
        let mut snake_head_entity = create_snake_head();
        snake_head_entity.set_name("SnakeEntityHead".to_string());
        // TODO: Do this linked list style until we implement query sorting
        (0..DEFAULT_TAIL_LENGTH)
            .fold(None, |prev_entity, index| {
                let mut tail_entity = create_snake_tail(index + 1);
                tail_entity.set_name(format!("SnakeEntityTail {}", tails.get_max_length()));
                if let Some(prev) = prev_entity {
                    tail_entity.child_of(prev);
                } else {
                    // Make first tail segment a child of the snake head
                    tail_entity.child_of_id(snake_head_entity.get_id());
                }
                Some(tail_entity)
        });
        Entity::named("CreateSnakeOnce");
    }
    create_food();
}
