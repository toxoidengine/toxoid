use sokol::gfx::Color;
use sokol::{app as sapp, gfx as sg};
use crate::bindings::*;

pub struct RenderTarget {
    width: u32,
    height: u32,
    // image: sg_image,
}

// Creates a framebuffer
pub fn create(width: u32, height: u32) -> RenderTarget {
    // let desc = sg_image_desc {
    //     width: width as i32,
    //     height: height as i32,
    //     pixel_format: sg_pixel_format::SG_PIXELFORMAT_RGBA8,
    //     ..Default::default()
    // };

    // let image = unsafe { sg_make_image(&desc) };

    RenderTarget {
        width,
        height,
        // image,
    }
}

pub fn blit(render_target: &RenderTarget, image: sg_image, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32) {
    // Assuming that sgp_draw_textured_rect is the equivalent of blit operation in sokol_gp
    let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
    let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
    unsafe {
        sgp_draw_textured_rect(0, dest_rect, src_rect);
    }
}

pub fn render(render_target: &RenderTarget, x: i32, y: i32) {
    // Assuming that sgp_viewport is the equivalent of render operation in sokol_gp
    unsafe {
        // sgp_viewport(x, y, render_target.width as i32, render_target.height as i32);
    }
}

pub fn resize(render_target: &mut RenderTarget, width: u32, height: u32) {
    render_target.width = width;
    render_target.height = height;
    // Assuming that sgp_viewport is the equivalent of resize operation in sokol_gp
    unsafe {
        sgp_viewport(0, 0, width as i32, height as i32);
    }
}

pub fn clear(render_target: &RenderTarget, x: i32, y: i32, width: i32, height: i32) {
    // Assuming that sgp_scissor and sgp_clear are the equivalent of clear operation in sokol_gp
    unsafe {
        sgp_scissor(x, y, width, height);
        sgp_clear();
    }
}

pub fn clear_canvas(x: i32, y: i32, width: i32, height: i32) {
    // Assuming that sgp_scissor and sgp_clear are the equivalent of clear_canvas operation in sokol_gp
    unsafe {
        sgp_scissor(x, y, width, height);
        sgp_clear();
    }
}

pub fn draw_filled_rect(rect: sgp_rect, color: Color) {
    unsafe {
        sgp_push_transform();
        sgp_set_color(color.r, color.g, color.b, color.a);
        sgp_translate(rect.x, rect.y);
        sgp_draw_filled_rect(0., 0., rect.w, rect.h);
        sgp_pop_transform();
    }
}

pub fn draw_line(ax: f32, ay: f32, bx: f32, by: f32) {
    unsafe {
        sgp_draw_line(ax, ay, bx, by);
    }
}