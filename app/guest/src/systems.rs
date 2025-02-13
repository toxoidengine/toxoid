use toxoid_api::*;
pub fn init() {
    println!("Systems initialized");
    System::dsl("TestChild, (ChildOf, $Parent), TestParent($Parent)", None, |iter| {
        iter.entities().iter().for_each(|entity| {
            println!("Entity: {}", entity.get_id());
            let parent = entity.parent();
            println!("Parent: {}", parent.get_id());
            let grand_parent = parent.parent();
            println!("Grand parent: {}", grand_parent.get_id());
        });
    }).build();
}