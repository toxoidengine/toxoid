#[cfg(not(target_os = "emscripten"))]
use crate::bindings::*;
#[cfg(target_os = "emscripten")]
use crate::bindings_x86::*;
use sokol::{app as sapp, gfx as sg, glue as sglue};
use toxoid_render::{Renderer2D, RenderTarget, Sprite};
use toxoid_api::{components::{Color, Position, Size, GameConfig}, World};
use std::any::Any;

pub struct SokolRenderer2D {
    pass_action: sg::PassAction,
}

pub struct SokolSprite {
    pub width: u32,
    pub height: u32,
    pub image: sg::Image,
}

pub struct SokolRenderTarget {
    pub sprite: Box<dyn Sprite>,
    pub depth_image: sg::Image,
    pub sampler: sg::Sampler,
    pub pass: sg::Pass
}

pub struct SpineOffscreenCtx {
    pub ctx: sspine_context,
    pub img: sg::Image,
    pub attachments: sg::Attachments,
    pub pass_action: sg::PassAction,
}

fn filter_from_c_int(value: u32) -> Option<sg::Filter> {
    match value {
        0 => Some(sg::Filter::Default),
        1 => Some(sg::Filter::Default),
        2 => Some(sg::Filter::Nearest),
        3 => Some(sg::Filter::Linear),
        4 => Some(sg::Filter::Num),
        _ => None, // Return None for undefined integer values
    }
}

fn wrap_from_c_int(value: u32) -> Option<sg::Wrap> {
    match value {
        0 => Some(sg::Wrap::Default),
        1 => Some(sg::Wrap::Repeat),
        2 => Some(sg::Wrap::ClampToEdge),
        3 => Some(sg::Wrap::ClampToBorder),
        4 => Some(sg::Wrap::MirroredRepeat),
        5 => Some(sg::Wrap::Num),
        _ => None, // Return None for undefined integer values
    }
}

impl Sprite for SokolSprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    /*
    fn drop(&self) {
        sg_destroy_image(&self.image);
    }
    */
}

impl RenderTarget for SokolRenderTarget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /*
    fn drop(&self) {
        &self.sprite.drop();
        sg_destroy_image(self.depth_image);
        sg_destroy_pass(self.pass);
        sg_destroy_sampler(self.sampler);
    }
    */
}

impl SokolRenderer2D {
    pub fn init_image(sgimage: sg_image, data: *const u8, size: usize) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut channels: i32 = 0;
        let image_data = unsafe {
            // Converts from PNG format to RGBA8 format
            stbi_load_from_memory(data as *const u8, size as core::ffi::c_int, &mut width, &mut height, &mut channels, 0)
        };
        let image_desc = sg::ImageDesc {
            width,
            height,
            pixel_format: sg::PixelFormat::Rgba8,
            data: sg::ImageData {
                subimage: [[sg::Range { ptr: image_data as *const core::ffi::c_void, size: (width * height * 4) as usize }; 16]; 6],
                ..Default::default()
            },
            ..Default::default()
        };

        sg::init_image(sg::Image { id: sgimage.id }, &image_desc);

        unsafe { stbi_image_free(image_data as *mut core::ffi::c_void) };
    }
    
    pub fn init_sampler(sgsampler: sg_sampler, min_filter: sg_filter, mag_filter: sg_filter, mipmap_filter: sg_filter, wrap_u: sg_wrap, wrap_v: sg_wrap, label: *const i8) {
        let sampler_desc = sg::SamplerDesc {
            min_filter: filter_from_c_int(min_filter.try_into().unwrap()).unwrap(),
            mag_filter: filter_from_c_int(mag_filter.try_into().unwrap()).unwrap(),
            mipmap_filter: filter_from_c_int(mipmap_filter.try_into().unwrap()).unwrap(),
            wrap_u: wrap_from_c_int(wrap_u.try_into().unwrap()).unwrap(),
            wrap_v: wrap_from_c_int(wrap_v.try_into().unwrap()).unwrap(),
            label,
            ..Default::default()
        };
        sg::init_sampler(sg::Sampler { id: sgsampler.id }, &sampler_desc);
    }
}

impl Renderer2D for SokolRenderer2D {
    fn new() -> Self {
        Self {
            pass_action: sg::PassAction::new()
        }
    }

    fn window_size() -> (u32, u32) {
        let game_config = World::get_singleton::<GameConfig>();
        (game_config.get_window_width(), game_config.get_window_height())
    }

    fn begin() {
        let game_config = World::get_singleton::<GameConfig>();
        let window_width = game_config.get_window_width() as i32;
        let window_height = game_config.get_window_height() as i32;
        let game_width = game_config.get_game_width() as f32;
        let game_height = game_config.get_game_height() as f32;
        
        // Calculate integer scale factor that maintains aspect ratio
        let scale_x = (window_width as f32 / game_width).floor();
        let scale_y = (window_height as f32 / game_height).floor();
        let scale_factor = scale_x.min(scale_y).max(1.0);
        
        // Calculate viewport dimensions
        let viewport_width = (game_width * scale_factor) as i32;
        let viewport_height = (game_height * scale_factor) as i32;
        
        // Center the viewport
        let viewport_x = (window_width - viewport_width) / 2;
        let viewport_y = (window_height - viewport_height) / 2;

        unsafe {
            // Clear to black
            sgp_begin(window_width, window_height);
            sgp_viewport(0, 0, window_width, window_height);
            sgp_project(0.0, window_width as f32, 0.0, window_height as f32);
            sgp_reset_color();
            sgp_set_color(0.0, 0.0, 0.0, 1.0);
            sgp_clear();
            
            // Set up pixel-perfect viewport
            sgp_viewport(viewport_x, viewport_y, viewport_width, viewport_height);
            sgp_project(0.0, game_width, 0.0, game_height);
            
            #[cfg(feature = "imgui")]
            {
                let desc = simgui_frame_desc_t {
                    width: viewport_width,
                    height: viewport_height,
                    delta_time: sapp::frame_duration(),
                    dpi_scale: sapp::dpi_scale(),
                };
                simgui_new_frame(&desc)
            }
        }
    }

    fn end() {
        let (window_width, window_height) = (sapp::width(), sapp::height());
            
        // Begin render pass
        sg::begin_pass(&sg::Pass {
            action: *crate::PASS_ACTION,
            swapchain: sglue::swapchain(),
            ..Default::default()
        });
        unsafe {
            // Draw regular 2D stuff
            sgp_flush(); 
            sgp_end();
            
            // Draw Spine layer
            // #[cfg(feature = "spine")] {
            //     let layer_transform = sspine_layer_transform {
            //         size: sspine_vec2 { 
            //             x: window_width as f32, 
            //             y: window_height as f32
            //         },
            //         origin: sspine_vec2 { 
            //             x: window_width as f32 * 0.5, 
            //             y: window_height as f32 * 0.5
            //         }
            //     };
            //     sspine_draw_layer(0, &layer_transform);
            // }

            // Render ImGui
            #[cfg(feature = "imgui")]
            {
                simgui_render();
            }
        }
        // End render pass
        sg::end_pass();
        sg::commit();
    }

    fn create_render_target(width: u32, height: u32) -> Box<dyn RenderTarget> {
        // Get swapchain info to match formats
        let swapchain = sglue::swapchain();

        // Create framebuffer image
        let image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            pixel_format: swapchain.color_format, // Match swapchain format
            sample_count: swapchain.sample_count, // Match swapchain sample count
            ..Default::default()
        };
        let image = sg::make_image(&image_desc);

        // Create framebuffer depth stencil
        let depth_image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            pixel_format: swapchain.depth_format, // Match swapchain depth format
            sample_count: swapchain.sample_count, // Match swapchain sample count
            ..Default::default()
        };
        let depth_image = sg::make_image(&depth_image_desc);

        // Create linear sampler
        let sampler_desc = sg::SamplerDesc {
            min_filter: sg::Filter::Linear,
            mag_filter: sg::Filter::Linear,
            wrap_u: sg::Wrap::ClampToEdge,
            wrap_v: sg::Wrap::ClampToEdge,
            ..Default::default()
        };
        let sampler = sg::make_sampler(&sampler_desc);
        

        // Create framebuffer pass
        let mut attachments_desc = sg::AttachmentsDesc::default();
        attachments_desc.colors[0].image = image;
        attachments_desc.depth_stencil.image = depth_image;
        let attachments = sg::make_attachments(&attachments_desc);
        let mut pass_action = sg::PassAction::default();
        pass_action.colors[0] = sg::ColorAttachmentAction {
            load_action: sg::LoadAction::Clear,
            store_action: sg::StoreAction::Store,
            clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            ..Default::default()
        };
        let fb_pass = sg::Pass {
            attachments,
            action: pass_action,
            ..Default::default()
        };

        // TODO: Error handling
        // let state_1 = sg::query_image_state(image);
        // let state_2 = sg::query_image_state(depth_image);
        // let state_3 = sg::query_sampler_state(sampler);

        // println!("Image state: {:?}", state_1);
        // println!("Depth image state: {:?}", state_2);
        // println!("Sampler state: {:?}", state_3);

        Box::new(SokolRenderTarget {
            sprite: Box::new(SokolSprite {
                width,
                height,
                image: sg::Image { id: image.id },
            }),
            depth_image: sg::Image { id: depth_image.id },
            sampler: sg::Sampler { id: sampler.id },
            pass: fb_pass
        })
    }

    fn create_sprite(data: *const u8, size: usize) -> Box<dyn Sprite> {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut channels: i32 = 0;
        let image_data = unsafe {
            // Converts from PNG format to RGBA8 format
            stbi_load_from_memory(data as *const u8, size as core::ffi::c_int, &mut width, &mut height, &mut channels, 0)
        };
        // Slice from image data from raw parts
        // let image_data_slice = unsafe { std::slice::from_raw_parts(image_data, (width * height * 4) as usize) };
        // println!("Image data: {:?}", image_data_slice);
        let image_desc = sg::ImageDesc {
            width,
            height,
            pixel_format: sg::PixelFormat::Rgba8,
            data: sg::ImageData {
                subimage: [[sg::Range { ptr: image_data as *const core::ffi::c_void, size: (width * height * 4) as usize }; 16]; 6],
                ..Default::default()
            },
            
            ..Default::default()
        };
        let image = sg::make_image(&image_desc);

        let sprite_boxed = Box::new(SokolSprite {
                width: width as u32,
                height: height as u32,
                image
        });
        sprite_boxed
    }
    
    fn begin_rt(destination: &Box<dyn RenderTarget>, dw: f32, dh: f32) {
        unsafe {
            // Set the framebuffer as the current render target
            let sokol_destination = destination.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
            sg::begin_pass(&sokol_destination.pass);

            // Initialize sokol-gp context for the render target
            sgp_begin(dw as i32, dh as i32);
            sgp_viewport(0, 0, dw as i32, dh as i32);
            // Flip the y-axis and maintain aspect ratio
            #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
            sgp_project(0.0, dw, dh, 0.0);  
            // Changed order of y coordinates
            // sgp_set_color(0., 0., 0., 0.);
            // sgp_clear();
            // sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
            sgp_reset_color();
            sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
        }
    }

    fn end_rt() {
        unsafe {
            // Flush the sokol-gp commands to the current render target
            sgp_flush();
            sgp_end();
            
            // End the render pass
            sg::end_pass();
        }
    }

    fn blit_sprite(source: &Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, destination: &Box<dyn RenderTarget>, dx: f32, dy: f32) {
        unsafe {      
            let sokol_source = source.as_any().downcast_ref::<SokolSprite>().unwrap();
        
            // Set the source image
            sgp_set_image(0, sg_image { id: sokol_source.image.id });
        
            // Add sampler configuration to prevent wrapping
            sgp_set_sampler(0, sg_sampler { 
                id: destination.as_any()
                    .downcast_ref::<SokolRenderTarget>()
                    .unwrap().sampler.id 
            });
            
            // Draw the source sprite onto the destination sprite
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn resize_sprite(sprite: &Box<dyn Sprite>, width: u32, height: u32) {
        // let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
        // let old_image = sokol_sprite.image;

        // // Create a new image with the desired dimensions
        // let new_image_desc = sg_image_desc {
        //     width: width as i32,
        //     height: height as i32,
        //     // Copy other parameters from the old image
        //     ..unsafe { sg_query_image_desc(old_image) }
        // };
        // let _new_image = unsafe { sg_make_image(&new_image_desc) };

        // // TODO: Copy old image data into the new image

        // // Replace the old image with the new one
        // // sokol_sprite.image = new_image;

        // // Destroy the old image
        // unsafe { sg_destroy_image(old_image) };
        unimplemented!("Resize sprite not implemented");
    }

    fn draw_sprite(sprite: &Box<dyn Sprite>, x: f32, y: f32) {
        unsafe {
            let (window_width, _) = SokolRenderer2D::window_size();
            // let scale_factor = window_width as f32 / crate::GAME_WIDTH as f32;
            let dest_rect = sgp_rect { 
                x: 0., 
                y: 0., 
                w: sprite.width() as f32, 
                h: sprite.height() as f32
            };
            let src_rect = sgp_rect { 
                x: 0., 
                y: 0., 
                w: sprite.width() as f32, 
                h: sprite.height() as f32 
            };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sg_image { id: sokol_sprite.image.id });
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn draw_render_target(
        rt_trait_object: &Box<dyn RenderTarget>,
        sx: f32, sy: f32, sw: f32, sh: f32,
        dx: f32, dy: f32, dw: f32, dh: f32,
        blend_mode: u8
    ) {
        unsafe {
            sgp_reset_color();
            if blend_mode == 0 {
                sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
            } else {
                sgp_set_blend_mode((blend_mode as u32).try_into().unwrap());
            }

            let sokol_source = rt_trait_object.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
            let sprite = sokol_source.sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
    
            // Define the source and destination rectangles in game coordinates
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let dest_rect = sgp_rect { x: dx, y: dy, w: dw, h: dh };

            sgp_set_image(0, sg_image { id: sprite.image.id });
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn draw_filled_rect(pos: &Position, size: &Size, color: &Color) {
        unsafe {
            let game_config = World::get_singleton::<GameConfig>();
            let window_width = game_config.get_window_width() as f32;
            let game_width = game_config.get_game_width() as f32;
            let min_width = game_config.get_min_window_width() as f32;
            
            let scale_factor = (window_width / game_width).max(min_width / game_width);
            
            sgp_reset_color();
            sgp_set_color(color.get_r(), color.get_g(), color.get_b(), color.get_a());
            sgp_draw_filled_rect(
                pos.get_x() as f32 * scale_factor, 
                pos.get_y() as f32 * scale_factor, 
                size.get_width() as f32 * scale_factor, 
                size.get_height() as f32 * scale_factor
            );
        }
    }

    fn draw_line(ax: f32, ay: f32, bx: f32, by: f32) {
        unsafe {
            let game_config = World::get_singleton::<GameConfig>();
            let window_width = game_config.get_window_width() as f32;
            let game_width = game_config.get_game_width() as f32;
            let min_width = game_config.get_min_window_width() as f32;
            
            let scale_factor = (window_width / game_width).max(min_width / game_width);
            
            sgp_draw_line(
                ax * scale_factor, 
                ay * scale_factor, 
                bx * scale_factor, 
                by * scale_factor
            );
        }
    }

    fn clear_sprite(sprite: &Box<dyn RenderTarget>, x: i32, y: i32, width: i32, height: i32) {
        let sokol_render_target = sprite.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
    
        unsafe {
            // The sgp_scissor function sets a scissor rectangle in the viewport. The scissor test is a per-sample operation performed after the fragment shader. It discards the fragment if the fragment's position lies outside the scissor rectangle. In other words, it restricts drawing to a certain rectangular area of the screen.
            // You need to call sgp_begin before you can set a scissor rectangle with sgp_scissor, and you need to call sgp_end when you're done.
            sgp_begin(width, height);
            // The sgp_project function sets the coordinate space boundary in the current viewport. It's used to define the 2D projection matrix for the rendering context. The parameters left, right, top, and bottom define the boundaries of the coordinate space. This function is typically used when you want to set up a specific 2D coordinate system for your rendering context.
            sgp_project(0., width as f32, height as f32, 0.);

            // Set the framebuffer as the current render target
            let pass_action = sg::PassAction {
                colors: [sg::ColorAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: sg::Color::new(),
                }; sg::MAX_COLOR_ATTACHMENTS],
                depth: sg::DepthAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: 0.0,
                },
                stencil: sg::StencilAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: 0,
                },
                ..Default::default()
            };
            sg::begin_pass(&sokol_render_target.pass);
    
            // Set a scissor rectangle to the desired area
            sgp_scissor(x, y, width, height);
    
            // Set the color to the clear color
            sgp_set_color(0., 0., 0., 0.); // Replace with your clear color
    
            // Draw a rectangle over the scissor area
            sgp_draw_filled_rect(x as f32, y as f32, width as f32, height as f32);
    
            // Reset the scissor rectangle to default
            sgp_reset_scissor();

            // Flush the draw commands to the 
            // The sgp_flush function dispatches the current Sokol GFX draw commands. It's used to ensure that all the draw commands that have been issued up to this point are sent to the GPU for rendering. This function doesn't end the current draw command queue, so you can continue issuing draw commands after calling sgp_flush.
            sgp_flush();

            // Finish the draw command queue, clearing it
            sgp_end();
    
            // End the pass to apply the drawing commands to the framebuffer
            sg::end_pass();
        }
    }

    fn clear_canvas(x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            sgp_scissor(x, y, width, height);
            sgp_clear();
            sgp_reset_scissor();
        }
    }
}