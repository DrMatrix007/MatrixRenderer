use wgpu::{
    Adapter, Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, InstanceDescriptor, Limits, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError, TextureUsages,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::pipelines::{PipelineRenderable, RenderConfig};

pub struct Renderer {
    surface: Surface,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    device: Device,
    _adapter: Adapter,
    pipelines: Vec<Box<dyn PipelineRenderable>>,
    background_color: Color,
}

impl Renderer {
    pub async fn new(window: &Window, background_color: Color) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: Default::default(),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(*surface_caps.formats.first().unwrap());

        let surface_config = SurfaceConfiguration {
            alpha_mode: surface_caps.alpha_modes.first().unwrap().to_owned(),
            present_mode: surface_caps.present_modes.first().unwrap().to_owned(),
            format: surface_format,
            height: size.height,
            width: size.width,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: Vec::new(),
        };

        surface.configure(&device, &surface_config);
        //let aspect =  surface_config.width as f32 / surface_config.height as f32;

        Self {
            surface,
            queue,
            surface_config,
            device,
            _adapter: adapter,
            background_color,
            pipelines: Vec::new(),
        }
    }
    pub fn add_pipeline(&mut self, p: Box<dyn PipelineRenderable>) {
        self.pipelines.push(p);
    }

    pub fn resize(&mut self, p: PhysicalSize<u32>) {
        if p.height * p.width == 0 {
            return;
        }
        self.surface_config.width = p.width;
        self.surface_config.height = p.height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                depth_stencil_attachment: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: true,
                    },
                    resolve_target: None,
                    view: &view,
                })],
            });
            for i in self.pipelines.iter_mut() {
                {
                    let mut args = RenderConfig {
                        queue: &self.queue,
                        pass: &mut render_pass,
                        config: &self.surface_config,
                    };
                    i.render(&mut args);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
