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

    let mut entity = Entity::new(None);
    entity.add::<TestComponent>();
    entity.add::<TestRelated>();
    let test_component = entity.get::<TestComponent>();
    test_component.set_test("Hello world!".to_string());

    let mut entity_2 = Entity::new(None);
    entity_2.add::<TestRelated>();
    entity_2.add_relationship(Relationship::Custom(TestRelationship::get_id()), entity);

    entity_2.relationship_entities(Relationship::Custom(TestRelationship::get_id())).iter().for_each(|relationship|  {
        println!("Relationship: {:?}", relationship.get_id());
    });

    // System::dsl("(TestRelationship, *)", None, 
    //     |_iter| {
    //         println!("Hello world!");
    //         Query::dsl_each("TestComponent", |iter| {
    //             iter.components::<TestComponent>(0).iter().for_each(|component| {
    //                 println!("Hello query!");
    //                 println!("QUery component: {:?}", component.get_test());
    //             });
    //         });
    //     })
    //     .named("TestSystem")
    //     .build();
}
