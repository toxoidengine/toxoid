use toxoid_api::*;

component! {
    Direction {
        direction: u8
    },
    Stats {
        score: u32,
        high_score: u32,
        tail_length: u32
    },
    Tails {
        max_length: u32,
        entities: Vec::<u64>
    },
    Head {
        foo: bool
    },
    Tail {
        foo: bool
    },
    Player {
        foo: bool
    },
    Food {
        foo: bool
    },
    FoodEntity {
        entity: u64
    }
}

pub fn init() {
    Direction::register();
    Stats::register();
    Tails::register();
    Tail::register();
    Player::register();
    Food::register();
    Head::register();
    FoodEntity::register();
    World::add_singleton::<FoodEntity>();
    World::add_singleton::<Direction>();
    World::add_singleton::<Tails>();
}