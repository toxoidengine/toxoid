
use toxoid_api::*;
use toxoid_sokol::SokolSprite;


pub fn init() {
    // Game Config
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    // Keyboard Input
    World::add_singleton::<KeyboardInput>();

    // Local Player
    World::add_singleton::<Networked>();
    
    // Parent entity
    let mut player_entity = Entity::new();
    player_entity.add::<Local>();
    player_entity.add::<Player>();
    player_entity.add::<Networked>();
    let mut local_player = World::get_singleton::<Networked>();
    local_player.set_entity_id(player_entity.get_id());
    // TODO: Make animation a child entity of player later
    #[cfg(target_os = "emscripten")]
    // let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    // let mut position = player_animation_entity.get::<Position>();
    #[cfg(target_os = "emscripten")]
    // player_animation_entity.add::<Local>();

    use toxoid_render::Renderer2D;
    use toxoid_sokol::SokolRenderer2D;

    crate::utils::load::load_sprite("assets/character.png", |entity: &mut Entity| {
        let mut position = entity.get::<Position>();
        position.set_x(100);
        position.set_y(100);

        let rt = SokolRenderer2D::create_render_target(500, 500);
        let sprite_component = entity.get::<Sprite>();
        let sprite_ptr = sprite_component.get_sprite().ptr as *mut SokolSprite;
        let sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(sprite_ptr) };
        // Use reborrowing to create a temporary reference to the Box<dyn Sprite>
        // This is unsafe because we're assuming the pointer is valid and correctly typed
        // let sprite_ref: &Box<dyn toxoid_render::Sprite> = unsafe { &*(sprite_ptr as *const Box<dyn toxoid_render::Sprite>) };

        SokolRenderer2D::begin_rt(&rt, 500., 500.);
        SokolRenderer2D::blit_sprite(&sprite, 0., 0., 100., 100., &rt, 0., 0.);
        
        SokolRenderer2D::blit_sprite(&sprite, 0., 0., 100., 100., &rt, 50., 50.);
        SokolRenderer2D::end_rt();
        
        // Create render target entity
        {
            let mut entity = Entity::new();
            entity.add::<Position>();
            entity.add::<Size>();
            entity.add::<RenderTarget>();

            let mut position = entity.get::<Position>();
            position.set_x(100);
            position.set_y(100);
            let mut size = entity.get::<Size>();
            size.set_width(500);
            size.set_height(500);
            let mut render_target = entity.get::<RenderTarget>();
            render_target.set_render_target(Pointer{ ptr: Box::into_raw(rt) as *mut c_void });

            entity.add::<Renderable>();
        }
        
        // Flag sprite as renderable for draw_sprite_system
        // entity.add::<Renderable>();

        // std::mem::forget(rt);
        std::mem::forget(sprite);
    });
}