use std::{any::TypeId, collections::HashMap, sync::Arc};

use crate::{
    pipelines::{
        bind_groups::{BindData, BindGroupLayoutContainer},
        buffers::Vertex,
        matrix_render_pipeline::{MatrixRenderPipeline, MatrixRenderPipelineArgs},
        shaders::ShaderConfig,
        texture::MatrixTexture,
    },
    shaders,
};
use matrix_engine::{
    components::{
        component::ComponentCollection,
        resources::{Resource, ResourceHolder},
    },
    dispatchers::{context::ResourceHolderManager, systems::AsyncSystem},
};
use matrix_engine::{dispatchers::context::Context, events::event_registry::EventRegistry};
use wgpu::{
    Backends, BindGroupLayout, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, Limits, Operations, PowerPreference, Queue, Surface, SurfaceConfiguration,
    SurfaceError, TextureUsages,
};
use winit::dpi::PhysicalSize;

use super::{
    camera::{CameraResource, CameraUniform},
    render_object::RenderObject,
    window::MatrixWindow,
};

pub struct RendererResourceArgs<'a> {
    pub window: &'a MatrixWindow,
    pub background_color: Color,
}

pub struct RendererResource {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    background_color: Color,
    bind_groups: HashMap<TypeId, Arc<BindGroupLayout>>,
}

impl RendererResource {
    pub fn new(args: RendererResourceArgs) -> Self {
        let size = args.window.size();

        let runtime =
            tokio::runtime::Runtime::new().expect("the runtime is needed for the adapter");

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(args.window.raw()) }.unwrap();
        let adapter = runtime
            .block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .unwrap();

        let (device, queue) = runtime
            .block_on(adapter.request_device(
                &DeviceDescriptor {
                    label: Some("RenderDevice"),
                    features: Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        Limits::downlevel_webgl2_defaults()
                    } else {
                        Limits::default()
                    },
                },
                None,
            ))
            .expect("the device and queue are needed");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as _,
            height: size.height as _,
            present_mode: surface_caps.present_modes[0],
            view_formats: vec![],
            alpha_mode: surface_caps.alpha_modes[0],
        };

        surface.configure(&device, &config);

        Self {
            config,
            device,
            queue,
            surface,
            background_color: args.background_color,
            bind_groups: Default::default(),
        }
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) {
        if size.width > 0 || size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn get_bind_group_layout<T: BindData + 'static>(&mut self) -> BindGroupLayoutContainer<T> {
        BindGroupLayoutContainer::from(
            self.bind_groups
                .entry(TypeId::of::<T>())
                .or_insert_with(|| {
                    T::create_layout("auto generated bind group layout", &self.device).into()
                })
                .clone(),
        )
    }
}

impl Resource for RendererResource {}

pub struct RendererSystem;

impl AsyncSystem for RendererSystem {
    type Query<'a> = (
        &'a EventRegistry,
        (
            &'a ResourceHolder<MatrixWindow>,
            &'a mut ResourceHolder<RendererResource>,
            &'a mut ResourceHolder<MainPipeline>,
            &'a mut ResourceHolder<CameraResource>,
        ),
        &'a ComponentCollection<RenderObject>,
    );

    fn run(
        &mut self,
        ctx: &Context,
        (events, (window_resource, render_resource, main_pipeline,camera_resource),objects): <Self as AsyncSystem>::Query<
            '_,
        >,
    ) {
        let Some(window_resource) = window_resource.get() else { return; };
        let render_resource = ctx.get_or_insert_resource_with(render_resource, || {
            RendererResource::new(RendererResourceArgs {
                window: window_resource,
                background_color: Color::WHITE,
            })
        });
        let main_pipeline = ctx.get_or_insert_resource_with(main_pipeline, || {
            MainPipeline::new(MatrixRenderPipelineArgs {
                device: &render_resource.device,
                shaders: shaders!(&render_resource.device, "shaders.wgsl", "main shaders"),
                shader_config: ShaderConfig {
                    fragment_main: "f_main".to_owned(),
                    vertex_main: "v_main".to_owned(),
                },
                pipe_label: "main pipeline",
                group_label: "main groups",
                surface_config: &render_resource.config,
                primitive_state: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
            })
        });
        let events = events.get_window_events(window_resource.id());
        if let Some(size) = events.is_resized() {
            render_resource.resize(size);
        }

        let camera_resource = ctx
            .get_or_insert_resource_with(camera_resource, || CameraResource::new(render_resource));

        camera_resource.update_buffer(render_resource.queue());
        {
            let s = window_resource.size();
            camera_resource.camera_mut().prespective.aspect = s.width as f32 / s.height as f32;
        }
        let current = render_resource.surface.get_current_texture();
        if let Ok(output) = current {
            let view = output.texture.create_view(&Default::default());

            let mut encoder =
                render_resource
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("main render encoder"),
                    });
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("main render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: wgpu::LoadOp::Clear(render_resource.background_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                main_pipeline.begin(&mut pass);
                for (_, data) in objects.iter() {
                    main_pipeline
                        .apply_groups(&mut pass, (data.texture_group(), camera_resource.group()));

                    main_pipeline.apply_buffer(&mut pass, data.buffer());

                    main_pipeline.draw(&mut pass, 0..data.buffer().len() as u32, 0..1);
                }
            }

            render_resource
                .queue
                .submit(std::iter::once(encoder.finish()));
            output.present();
        } else if let Err(err) = current {
            match err {
                SurfaceError::Lost => {
                    render_resource.resize(&window_resource.size());
                }
                _ => {
                    ctx.quit();
                }
            };
        }
    }
}

pub(super) type MainPipeline = MatrixRenderPipeline<Vertex, ((MatrixTexture,), (CameraUniform,))>;
