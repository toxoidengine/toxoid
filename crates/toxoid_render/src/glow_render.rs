// 2D sprite renderer with automatic geometry batching.
// This renderer can efficiently draw large numbers of 2
// 2D sprites with varying positions, sizes, rotations, colors, and textures.
use std::collections::HashMap;

use glow::{self, HasContext};

#[derive(Default)]
struct Texture {
    id: glow::Texture,
    width: u32,
    height: u32,
}

pub struct Sprite {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub rotation: f32,
    pub texture: Texture,
}

pub struct SpriteRenderer {
    context: glow::Context,
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
    textures: HashMap<glow::Texture, Texture>,
    batches: HashMap<(glow::Texture, (f32, f32, f32)), Vec<Sprite>>,
}

impl SpriteRenderer {
    pub fn new(context: glow::Context) -> Self {
        unsafe {
            let program = Self::create_program(&context);
            let vao = context.create_vertex_array().unwrap();
            let vbo = context.create_buffer().unwrap();
            let ebo = context.create_buffer().unwrap();

            context.bind_vertex_array(Some(vao));
            context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let stride = std::mem::size_of::<f32>() * 8;
            let position_offset = 0;
            let texcoord_offset = std::mem::size_of::<f32>() * 2;
            let size_offset = std::mem::size_of::<f32>() * 4;
            let rotation_offset = std::mem::size_of::<f32>() * 6;

            context.enable_vertex_attrib_array(0);
            context.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                stride as i32,
                position_offset as i32,
            );

            context.enable_vertex_attrib_array(1);
            context.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                stride as i32,
                texcoord_offset as i32,
            );

            context.enable_vertex_attrib_array(2);
            context.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                stride as i32,
                size_offset as i32,
            );

            context.enable_vertex_attrib_array(3);
            context.vertex_attrib_pointer_f32(
                3,
                1,
                glow::FLOAT,
                false,
                stride as i32,
                rotation_offset as i32,
            );

            context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            context.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                &[0, 1, 2, 0, 2, 3],
                glow::STATIC_DRAW,
            );

            Self {
                context,
                program,
                vao,
                vbo,
                ebo,
                textures: HashMap::new(),
                batches: HashMap::new(),
            }
        }
    }

    fn create_program(context: &glow::Context) -> glow::Program {
        let vs_src = r#"
        #version 330 core
        
        layout (location = 0) in vec2 position;
        layout (location = 1) in vec2 texcoord;
        layout (location = 2) in vec2 size;
        layout (location = 3) in float rotation;
        
        uniform mat4 projection;
        
        out vec2 frag_texcoord;
        
        void main()
        {
            mat4 model = mat4(1.0);
            model = translate
            model = translate(model, vec3(position, 0.0));
            model = rotate(model, rotation, vec3(0.0, 0.0, 1.0));
            model = scale(model, vec3(size, 1.0));
            
            gl_Position = projection * model * vec4(position, 0.0, 1.0);
            frag_texcoord = texcoord;
        }
        "#;

        let fs_src = r#"
        #version 330 core
        
        in vec2 frag_texcoord;
        
        uniform sampler2D tex;
        
        out vec4 frag_color;
        
        void main()
        {
            frag_color = texture(tex, frag_texcoord);
        }
        "#;

        let program = unsafe {
            let vs = context.create_shader(glow::VERTEX_SHADER).unwrap();
            context.shader_source(vs, vs_src);
            context.compile_shader(vs);
            Self::print_shader_errors(context, vs);

            let fs = context.create_shader(glow::FRAGMENT_SHADER).unwrap();
            context.shader_source(fs, fs_src);
            context.compile_shader(fs);
            Self::print_shader_errors(context, fs);

            let program = context.create_program().unwrap();
            context.attach_shader(program, vs);
            context.attach_shader(program, fs);
            context.link_program(program);
            Self::print_program_errors(context, program);

            program
        };

        program
    }

    fn print_shader_errors(context: &glow::Context, shader: glow::Shader) {
        if !unsafe { context.get_shader_compile_status(shader) } {
            let log = unsafe { context.get_shader_info_log(shader) };
            eprintln!("Shader compile error: {}", log);
        }
    }

    fn print_program_errors(context: &glow::Context, program: glow::Program) {
        if !unsafe { context.get_program_link_status(program) } {
            let log = unsafe { context.get_program_info_log(program) };
            eprintln!("Program link error: {}", log);
        }
    }

    pub fn draw(&mut self, sprites: &[Sprite], projection: &glm::Mat4) {
        for sprite in sprites {
            let texture = sprite.texture.id;
            let size = sprite.size;
            let rotation = sprite.rotation;

            let batch = self
                .batches
                .entry((texture, (size.0, size.1, rotation)))
                .or_insert_with(|| Vec::new());
            batch.push(sprite.clone());
        }

        self.flush(projection);
    }

    fn flush(&mut self, projection: &glm::Mat4) {
        if self.batches.is_empty() {
            return;
        }

        let mut texture_units = Vec::new();
        let mut vbo_data = Vec::new();
        let mut indices = Vec::new();
        let mut offset = 0;

        for ((texture, (width, height, rotation)), sprites) in &self.batches {
            let texture = *texture;
            let texture_info = self.textures.get(&texture).unwrap();

            if !texture_units.contains(&texture_info.id) {
                texture_units.push(texture_info.id);
            }

            for sprite in sprites {
                let x = sprite.position.0;
                let y = sprite.position.1;
                let w = sprite.size.0;
                let h = sprite.size.1;
                let angle = sprite.rotation;

                let x0 = -w / 2.0;
                let x1 = w / 2.0;
                let y0 = -h / 2.0;
                let y1 = h / 2.0;

                let (sin_t, cos_t) = angle.sin_cos();

                let x0r = x0 * cos_t - y0 * sin_t;
                let y0r = x0 * sin_t + y0 * cos_t;
                let x1r = x1 * cos_t - y0 * sin_t;
                let y1r = x1 * sin_t + y0 * cos_t;
                let x2r = x1 * cos_t - y1 * sin_t;
                let y2r = x1 * sin_t + y1 * cos_t;
                let x3r = x0 * cos_t - y1 * sin_t;
                let y3r = x0 * sin_t + y1 * cos_t;

                let u0 = sprite.texcoords.0;
                let v0 = sprite.texcoords.1;
                let u1 = sprite.texcoords.2;
                let v1 = sprite.texcoords.3;

                let (r, g, b, a) = sprite.color.to_normalized();

                vbo_data.extend_from_slice(&[
                    x + x0r,
                    y + y0r,
                    u0,
                    v0,
                    width,
                    height,
                    rotation,
                    r,
                    g,
                    b,
                    a,
                    x + x1r,
                    y + y1r,
                    u1,
                    v0,
                    width,
                    height,
                    rotation,
                    r,
                    g,
                    b,
                    a,
                    x + x2r,
                    y + y2r,
                    u1,
                    v1,
                    width,
                    height,
                    rotation,
                    r,
                    g,
                    b,
                    a,
                    x + x3r,
                    y + y3r,
                    u0,
                    v1,
                    width,
                    height,
                    rotation,
                    r,
                    g,
                    b,
                    a,
                ]);

                indices.extend_from_slice(&[
                    offset,
                    offset + 1,
                    offset + 2,
                    offset + 2,
                    offset + 3,
                    offset,
                ]);

                offset += 4;
            }
        }

        let texture_units_len = texture_units.len();
        let max_texture_units = self.gl.get_parameter_i32(glow::MAX_TEXTURE_IMAGE_UNITS) as usize;
        let program = self.program;

        unsafe {
            self.gl.use_program(Some(program));

            let projection_loc = self.gl.get_uniform_location(program, "projection");
            self.gl.uniform_matrix_4_f32_slice(
                projection_loc.as_ref(),
                false,
                projection.as_slice(),
            );

            self.gl.bind_vertex_array(Some(self.vao));

            for (i, texture_unit) in texture_units.iter().enumerate() {
                let uniform_name = format!("tex{}", i);
                let uniform_loc = self.gl.get_uniform_location(program, uniform_name.as_str());

                self.gl.active_texture(glow::TEXTURE0 + i as u32);
                self.gl.bind_texture(glow::TEXTURE_2D, Some(*texture_unit));

                self.gl.uniform_1_i32(uniform_loc.as_ref(), i as i32);
            }

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                vbo_data.align_to::<u8>().1,
                glow::STREAM_DRAW,
            );

            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                indices.align_to::<u8>().1,
                glow::STREAM_DRAW,
            );

            let stride = mem::size_of::<f32>() * 11;

            self.gl
                .vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
            self.gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                stride,
                mem::size_of::<f32>() * 2,
            );
            self.gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                stride,
                mem::size_of::<f32>() * 4,
            );
            self.gl.vertex_attrib_pointer_f32(
                3,
                1,
                glow::FLOAT,
                false,
                stride,
                mem::size_of::<f32>() * 6,
            );
            self.gl.vertex_attrib_pointer_f32(
                4,
                4,
                glow::FLOAT,
                false,
                stride,
                mem::size_of::<f32>() * 7,
            );

            self.gl.enable_vertex_attrib_array(0);
            self.gl.enable_vertex_attrib_array(1);
            self.gl.enable_vertex_attrib_array(2);
            self.gl.enable_vertex_attrib_array(3);
            self.gl.enable_vertex_attrib_array(4);

            self.gl.draw_elements_instanced(
                glow::TRIANGLES,
                indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
                instances as i32,
            );

            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);

            for i in 0..texture_units_len {
                self.gl.active_texture(glow::TEXTURE0 + i as u32);
                self.gl.bind_texture(glow::TEXTURE_2D, None);
            }

            self.gl.use_program(None);
        }
    }
}
