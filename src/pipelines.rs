use wgpu::{
    include_wgsl, util::DeviceExt, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferUsages, ColorTargetState,
    ColorWrites, Device, Face, FragmentState, FrontFace, MultisampleState, PolygonMode,
    PrimitiveTopology, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor, Sampler,
    ShaderModule, ShaderStages, TextureFormat, TextureView, VertexBufferLayout,
};

use crate::{
    cameras::{Camera, CameraUniform},
    drawables::{BufferData, Drawable2D},
    math::{matrices::Matrix, vectors::Vector3D},
    vertex::Vertex,
};

pub struct RenderConfig<'a, 'b> {
    pub queue: &'a Queue,
    pub pass: &'b mut RenderPass<'a>,
}

pub struct PipelineConfig<'a, R: PipelineRenderer> {
    pub device: &'a Device,
    pub label: Option<&'a str>,
    pub primitive: PrimitiveConfig,
    pub fragment: FragmentConfig<'a>,
    pub vertex: VertexConfig<'a>,
    pub renderer: R,
}
#[derive(Clone, Copy)]
pub struct VertexConfig<'a> {
    pub entry_point: &'a str,
    pub module: &'a ShaderModule,
    pub buffers: &'a [VertexBufferLayout<'a>],
}

#[derive(Clone, Copy)]
pub struct FragmentConfig<'a> {
    pub entry_point: &'a str,
    pub module: &'a ShaderModule,
    pub format: TextureFormat,
}

#[derive(Clone, Copy)]
pub struct PrimitiveConfig {
    pub topology: PrimitiveTopology,
    pub front_face: FrontFace,
    pub unclipped_depth: bool,
    pub cull_mode: Option<Face>,
    pub polygon_mode: PolygonMode,
}
pub struct Pipeline<Renderer: PipelineRenderer<Drawable = Item>, Item: ?Sized> {
    render_pipeline: RenderPipeline,
    drawables: Vec<Box<Item>>,
    renderer: Renderer,
}

pub trait PipelineRenderer {
    type Drawable: ?Sized;

    fn get_bind_group_layouts(&self) -> Vec<&BindGroupLayout>;
    fn render<'a, 'b>(
        &'a mut self,
        render_config: &mut RenderConfig<'a, 'b>,
        drawables: &'a [Box<Self::Drawable>],
    );
}

impl<Renderer: PipelineRenderer<Drawable = Item>, Item: ?Sized> Pipeline<Renderer, Item> {
    pub fn new(conf: PipelineConfig<Renderer>) -> Self {
        let dev = conf.device;
        let binds_group_layout = conf.renderer.get_bind_group_layouts();
        let pipe_layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: binds_group_layout.as_slice(),
            push_constant_ranges: &[],
        });

        let pipe = dev.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipe_layout),
            vertex: wgpu::VertexState {
                module: conf.vertex.module,
                entry_point: conf.vertex.entry_point,
                buffers: conf.vertex.buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology: conf.primitive.topology,
                strip_index_format: None,
                front_face: conf.primitive.front_face,
                cull_mode: conf.primitive.cull_mode,
                unclipped_depth: conf.primitive.unclipped_depth,
                polygon_mode: conf.primitive.polygon_mode,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                alpha_to_coverage_enabled: false,
                count: 1,
                mask: !0,
            },
            fragment: Some(FragmentState {
                module: conf.fragment.module,
                entry_point: conf.fragment.entry_point,
                targets: &[Some(ColorTargetState {
                    format: conf.fragment.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            render_pipeline: pipe,
            drawables: Vec::new(),
            renderer: conf.renderer,
        }
    }
    pub fn add_drawable(&mut self, t: Box<Item>) {
        self.drawables.push(t);
    }
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}

pub trait PipelineRenderable {
    fn render<'a, 'b>(&'a mut self, render_conf: &mut RenderConfig<'a, 'b>);
}

impl<Renderer: PipelineRenderer<Drawable = Item>, Item: ?Sized> PipelineRenderable
    for Pipeline<Renderer, Item>
{
    fn render<'a, 'b>(&'a mut self, render_conf: &mut RenderConfig<'a, 'b>) {
        render_conf.pass.set_pipeline(&self.render_pipeline);
        self.renderer.render(render_conf, self.drawables.as_slice());
    }
}

pub struct Renderer2D {
    texture_group_layout: BindGroupLayout,
    camera_group_layout: BindGroupLayout,
    camera_bind_group: BindGroup,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
}

impl Renderer2D {
    pub fn new_pipeline(
        device: &Device,
        format: TextureFormat,
        camera: Camera,
    ) -> Pipeline<Self, <Self as PipelineRenderer>::Drawable> {
        let shader = device.create_shader_module(include_wgsl!("./simple_texture_shader.wgsl"));

        let mut camera_uniform = CameraUniform::default();
        camera_uniform.from_camera(&camera);
        let t_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    visibility: ShaderStages::FRAGMENT,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    visibility: ShaderStages::FRAGMENT,
                },
            ],
        });
        let c_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: ShaderStages::VERTEX,
            }],
        });
        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let c_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &c_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });
        let pipeline = Pipeline::<Renderer2D, dyn Drawable2D>::new(PipelineConfig {
            device,
            fragment: FragmentConfig {
                entry_point: "fs_main",
                format,
                module: &shader,
            },
            vertex: VertexConfig {
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                module: &shader,
            },
            label: None,
            primitive: PrimitiveConfig {
                cull_mode: Some(Face::Back),
                front_face: FrontFace::Ccw,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                topology: PrimitiveTopology::TriangleList,
            },
            renderer: Renderer2D {
                camera,
                camera_bind_group: c_group,
                camera_uniform,
                texture_group_layout: t_layout,
                camera_group_layout: c_layout,
                camera_buffer: camera_uniform_buffer,
            },
        });
        pipeline
    }

    pub fn create_texture_group(&self, d: &Device, t: &TextureView, s: &Sampler) -> BindGroup {
        d.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.texture_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(t),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(s),
                },
            ],
        })
    }
}

impl PipelineRenderer for Renderer2D {
    type Drawable = dyn Drawable2D;

    fn render<'a, 'b>(
        &'a mut self,
        render_config: &mut RenderConfig<'a, 'b>,
        drawables: &'a [Box<Self::Drawable>],
    ) {
        *self.camera.eye.x_mut() -= 0.1;
        self.camera_uniform.from_camera(&self.camera);
        render_config.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        for item in drawables.iter() {
            let BufferData {
                index_buffer,
                index_format,
                vertex_buffer,
            } = item.get_vertex_buffer();

            render_config
                .pass
                .set_bind_group(0, item.get_texture_group(), &[]);

            render_config
                .pass
                .set_bind_group(1, &self.camera_bind_group, &[]);

            render_config
                .pass
                .set_vertex_buffer(0, vertex_buffer.slice(..));

            render_config
                .pass
                .set_index_buffer(index_buffer.slice(..), index_format);

            render_config
                .pass
                .draw_indexed(item.get_verticies_range(), 0, 0..1);
        }
    }

    fn get_bind_group_layouts(&self) -> Vec<&BindGroupLayout> {
        vec![&self.texture_group_layout, &self.camera_group_layout]
    }
}
