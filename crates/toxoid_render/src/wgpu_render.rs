// 2D sprite renderer with automatic geometry / draw call batching.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                },
            ],
        }
    }
}

struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    async fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8]) -> Self {
        let img = image::load_from_memory(bytes).unwrap();
        let rgba = img.as_rgba8().unwrap();

        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
            label: None,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

struct Sprite {
    position: [f32; 2],
    size: [f32; 2],
    tex_coords: [f32; 4],
}

impl Sprite {
    fn vertices(&self) -> [Vertex; 4] {
        let [x, y] = self.position;
        let [w, h] = self.size;
        let [u0, v0, u1, v1] = self.tex_coords;

        [
            Vertex {
                position: [x, y],
                tex_coords: [u0, v0],
            },
            Vertex {
                position: [x + w, y],
                tex_coords: [u1, v0],
            },
            Vertex {
                position: [x + w, y + h],
                tex_coords: [u1, v1],
            },
            Vertex {
                position: [x, y + h],
                tex_coords: [u0, v1],
            },
        ]
    }
}

struct Batch {
    texture: Texture,
    sprites: Vec<Sprite>,
}

impl Batch {
    fn new(texture: Texture) -> Self {
        Self {
            texture,
            sprites: Vec::new(),
        }
    }

    fn push(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }
}

struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    batches: Vec<Batch>,
}

impl Renderer {
    async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));
    }

    fn create_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_descriptor.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        pipeline
    }

    fn create_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: VERTEX_DATA.len() as u64,
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: INDEX_DATA.len() as u64,
            usage: wgpu::BufferUsage::INDEX,
        });

        device
            .queue()
            .write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&VERTEX_DATA));

        device
            .queue()
            .write_buffer(&index_buffer, 0, bytemuck::cast_slice(&INDEX_DATA));

        (vertex_buffer, index_buffer)
    }

    fn create_bind_group(device: &wgpu::Device, texture: &Texture) -> wgpu::BindGroup {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        let texture_view = texture.create_view(device);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &Self::RENDER_PIPELINE_LAYOUT
                .bind_group_layouts
                .get(0)
                .unwrap(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
        });

        pub struct SpriteRenderer {
            pipeline: wgpu::RenderPipeline,
            vertex_buffer: wgpu::Buffer,
            index_buffer: wgpu::Buffer,
            bind_group: wgpu::BindGroup,
            sprites: Vec<Sprite>,
            max_sprites: usize,
        }

        impl SpriteRenderer {
            pub fn new(device: &wgpu::Device, max_sprites: usize) -> Self {
                let pipeline = SpriteRenderer::create_render_pipeline(device);
                let (vertex_buffer, index_buffer) = SpriteRenderer::create_buffers(device);
                let texture =
                    Texture::from_bytes(device, include_bytes!("../assets/texture.png")).unwrap();
                let bind_group = SpriteRenderer::create_bind_group(device, &texture);

                Self {
                    pipeline,
                    vertex_buffer,
                    index_buffer,
                    bind_group,
                    sprites: Vec::with_capacity(max_sprites),
                    max_sprites,
                }
            }

            pub fn add_sprite(&mut self, sprite: Sprite) {
                if self.sprites.len() < self.max_sprites {
                    self.sprites.push(sprite);
                }
            }

            pub fn render(&self, encoder: &mut wgpu::CommandEncoder) {
                if self.sprites.is_empty() {
                    return;
                }

                let vertices: Vec<Vertex> = self
                    .sprites
                    .iter()
                    .flat_map(|sprite| sprite.vertices())
                    .collect();

                let indices: Vec<u16> = self
                    .sprites
                    .iter()
                    .flat_map(|sprite| sprite.indices())
                    .collect();

                let num_indices = indices.len() as u32;

                let vertex_buffer = self
                    .vertex_buffer
                    .slice(..(vertices.len() * mem::size_of::<Vertex>()) as u64);
                let index_buffer = self
                    .index_buffer
                    .slice(..(num_indices * mem::size_of::<u16>()) as u64);

                let mut staging_buffer = wgpu::BufferInitDescriptor {
                    label: Some("Staging Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsage::MAP_WRITE | wgpu::BufferUsage::COPY_SRC,
                }
                .create_buffer(&self.pipeline.device);

                staging_buffer.write(bytemuck::cast_slice(&vertices));

                let mut commands = Vec::new();
                let mut offset: u32 = 0;

                for sprite in &self.sprites {
                    commands.push(wgpu::RenderPassColorAttachment {
                        view: &sprite.target.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    });

                    let num_indices = sprite.num_indices() as u32;
                    let draw = wgpu::RenderPassDrawIndexed {
                        indices: offset..offset + num_indices,
                        instances: 0..1,
                        vertex_offset: 0,
                    };

                    offset += num_indices;

                    commands.push(draw);
                }

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Sprite Render Pass"),
                    color_attachments: &commands,
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer);
                render_pass.set_index_buffer(index_buffer, 0, 0);
                render_pass.set_bind_group(0, &self.bind_group, &[]);

                for sprite in &self.sprites {
                    let num_indices = sprite.num_indices() as u32;
                    render_pass.set_scissor_rect(
                        sprite.viewport.x as u32,
                        sprite.viewport.y as u32,
                        sprite.viewport.width as u32,
                        sprite.viewport.height as u32,
                    );
                    render_pass.draw_indexed(offset..offset + num_indices, 0, 0..1);

                    offset += num_indices;
                }
            }
        }
    }
}
