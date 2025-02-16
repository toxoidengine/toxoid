
use toxoid_api::*;

component! {
    // TestChild {
    //     test: String,
    // },
    // TestParent {
    //     test: String,
    // },
    // TestGrandParent {
    //     test: String,
    // },
    TestComponent {
        test: String,
    },
    TestRelated {
        related: u64,
    },
    TestRelationship {}
}

pub fn init() {
    println!("Components initialized");
   
    TestComponent::register();
    TestRelated::register();
    TestRelationship::register();

    let mut test_component = Entity::new(None);
    test_component.add::<TestComponent>();

    let mut test_related = Entity::new(None);
    test_related.add::<TestRelated>();

    let mut entity = Entity::new(None);
    entity.add::<TestComponent>();
    let mut entity_2 = Entity::new(None);
    entity_2.add::<TestRelated>();
    entity_2.add_relationship(Relationship::Custom(TestRelationship::get_id()), entity);

    System::dsl("(TestRelationship, *)", None, |_iter| {
        println!("Hello world!");
    })
        .named("TestSystem")
        .build();
}