use toxoid_api::*;
use crate::prefabs::*;

pub fn init() {
    // Create render target entity
    let mut rt_entity = render_target(800, 600);
    // Create renderable entity
    rt_entity.add::<Renderable>();
}
