use matrix_engine::{
    components::resources::{Resource, ResourceHolder},
    dispatchers::systems::AsyncSystem,
    events::Events,
};
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    Limits, Operations, PowerPreference, Queue, Surface, SurfaceConfiguration, SurfaceError,
    TextureUsages,
};
use winit::dpi::PhysicalSize;

use super::window::MatrixWindow;

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
        }
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) {
        if size.width > 0 || size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

impl Resource for RendererResource {}

pub struct RendererSystem;

impl AsyncSystem for RendererSystem {
    type Query<'a> = (
        &'a mut ResourceHolder<RendererResource>,
        &'a ResourceHolder<MatrixWindow>,
        &'a Events,
    );

    fn run(
        &mut self,
        args: &matrix_engine::dispatchers::systems::SystemArgs,
        (render_resource, window_resource, events): <Self as AsyncSystem>::Query<'_>,
    ) {
        let Some(window_resource) = window_resource.get() else { return; };
        let render_resource = render_resource.get_or_insert_with(|| {
            RendererResource::new(RendererResourceArgs {
                window: window_resource,
                background_color: Color::WHITE,
            })
        });
        let events = events.get_window_events(window_resource.id());
        if let Some(size) = events.is_resized() {
            render_resource.resize(size);
        }
        match self.render(render_resource) {
            Err(SurfaceError::Lost) => {
                render_resource.resize(&window_resource.size());
            }
            Err(_) => {
                args.stop();
            }
            _ => {}
        };
    }
}

impl RendererSystem {
    fn render(&mut self, w: &RendererResource) -> Result<(), SurfaceError> {
        let output = w.surface.get_current_texture()?;

        let view = output.texture.create_view(&Default::default());

        let mut encoder = w.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("main render encoder"),
        });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(w.background_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        w.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
