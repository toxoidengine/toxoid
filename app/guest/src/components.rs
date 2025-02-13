
use toxoid_api::*;

component! {
    TestChild {
        test: String,
    },
    TestParent {
        test: String,
    },
    TestGrandParent {
        test: String,
    },
}

pub fn init() {
    println!("Components initialized");
    TestChild::register();
    TestParent::register();
    TestGrandParent::register();

    let mut test_child = Entity::new(None);
    test_child.add::<TestChild>();
    let mut test_parent = Entity::new(None);
    test_parent.add::<TestParent>();
    let mut test_grand_parent = Entity::new(None);
    test_grand_parent.add::<TestGrandParent>();

    test_child.child_of_id(test_parent.get_id());
    test_parent.child_of_id(test_grand_parent.get_id());
}