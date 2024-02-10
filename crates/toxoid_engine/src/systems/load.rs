use toxoid_api::*;

#[cfg(target_os = "emscripten")]
pub fn load_sprite_system(iter: &mut Iter) {
    // let entities = iter.entities();
    // entities
    //     .iter_mut()
    //     .for_each(|entity| {
    //         let sprite = entity.get::<Sprite>();

    //         println!("Sprite loading!");
    //         entity.remove::<Loadable>();
            
    //         use crate::utils::load::*;

    //         let filename = sprite.get_filename();
    //         fetch(filename, image_load_cb, entity as *mut Entity as *mut core::ffi::c_void, );
    //     });
}


#[cfg(target_os = "emscripten")]
pub fn load_bone_animation_system(iter: &mut Iter) {
    // let entities = iter.entities();
    // entities
    //     .iter_mut()
    //     .for_each(|entity| {
    //         // println!("Animation loading!");
    //         use crate::utils::load::*;

    //         entity.remove::<Loadable>();

    //         let skeleton = entity.get::<Skeleton>();
    //         let atlas = entity.get::<Atlas>();
    //         let skeleton_filename = skeleton.get_skeleton_filename();
    //         let atlas_filename = atlas.get_atlas_filename();
            
    //         // fetch(skeleton_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
    //         // fetch(atlas_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
    //     });
} 