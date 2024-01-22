use toxoid_api::*;

pub fn load_sprite_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let sprite = entity.get::<Sprite>();

                println!("Sprite loading!");
                entity.remove::<Loadable>();
                
                use crate::utils::load::*;

                let filename = sprite.get_filename();
                fetch(filename, entity as *mut Entity as *mut core::ffi::c_void, image_load_success, image_load_fail);
            });
    }
}

pub fn load_bone_animation_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                // println!("Animation loading!");
                use crate::utils::load::*;

                entity.remove::<Loadable>();

                let skeleton = entity.get::<Skeleton>();
                let atlas = entity.get::<Atlas>();
                let skeleton_filename = skeleton.get_skeleton_filename();
                let atlas_filename = atlas.get_atlas_filename();
                
                fetch(skeleton_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
                fetch(atlas_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
            });
    }
} 