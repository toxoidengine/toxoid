use toxoid_api::*;

component! {
    // Components
    // Tags
    Head {},
    Tail {},
    Player {},
    Food {},
    // Singleton
    FoodEntity {
        entity: EcsEntityT
    },
    Direction {
        direction: u8
    },
    Tails {
        max_length: u32
    },
    Stats {
        score: u32,
        high_score: u32,
        tail_length: u32
    },
}

pub fn init() {
    // Tags
    Head::register();
    Tail::register();
    Player::register();
    Food::register();

    // Singletons
    FoodEntity::register();
    Direction::register();
    Tails::register();
    Stats::register();

    // Add singletons
    World::add_singleton::<FoodEntity>();
    World::add_singleton::<Direction>();
    World::add_singleton::<Tails>();
    World::add_singleton::<Stats>();
}