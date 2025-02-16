use toxoid_api::*;

// Defining components
component! {
    TestComponent {
        test: String,
    },
    TestRelated {
        related: u64,
    },
    TestRelationship {}
}

// Initialization function
pub fn init() {
    println!("Components initialized");

    // Register components
    TestComponent::register();
    TestRelated::register();
    TestRelationship::register();

    // Create and add components to entities
    let mut test_component = Entity::new(None);
    test_component.add::<TestComponent>();

    let mut test_related = Entity::new(None);
    test_related.add::<TestRelated>();

    // Establish relationships
    let mut entity = Entity::new(None);
    entity.add::<TestComponent>();
    let mut entity_2 = Entity::new(None);
    entity_2.add::<TestRelated>();
    entity_2.add_relationship(Relationship::Custom(TestRelationship::get_id()), entity);

    // Define system behavior
    System::dsl("(TestRelationship, *)", None, |_iter| {
        println!("Hello world!");
    })
    .named("TestSystem")
    .build();
}
