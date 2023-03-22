use std::sync::atomic::Ordering;

use matrix_engine::systems::{System, SystemArgs, SystemRunner};
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    Limits, Operations, PowerPreference, Queue, Surface, SurfaceConfiguration, SurfaceError,
    TextureUsages,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::{run_return::EventLoopExtRunReturn, windows::EventLoopBuilderExtWindows},
    window::{Window, WindowBuilder},
};

pub struct WindowRendererSystemArgs {
    pub size: PhysicalSize<u32>,
    pub name: String,
    pub color: Color,
}

pub struct WindowRendererSystem {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    window: Window,
    background_color: Color,
}

impl WindowRendererSystem {
    pub fn new(args: WindowRendererSystemArgs, event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title(args.name)
            .with_inner_size(args.size)
            .build(event_loop)
            .expect("we should get the window");

        let size = window.inner_size();

        let runtime =
            tokio::runtime::Runtime::new().expect("the runtime is needed for the adapter");

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

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

        Self {
            config,
            window,
            device,
            queue,
            surface,
            background_color: args.color,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 || size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    fn render(&mut self, _args: &mut SystemArgs) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl System for WindowRendererSystem {
    fn update(&mut self, args: &mut SystemArgs) {
        match self.render(args) {
            Err(SurfaceError::Lost) => {
                self.resize(self.window.inner_size());
            }
            Err(_) => {
                args.stop();
            }
            _ => {}
        };
    }
}

pub struct WindowRendererRunner {
    args: WindowRendererSystemArgs,
}

impl WindowRendererRunner {
    pub fn new(args: WindowRendererSystemArgs) -> Self {
        Self { args }
    }
}

impl SystemRunner for WindowRendererRunner {
    fn run(self, args: matrix_engine::systems::SystemRunnerArgs) {
        let (mut args, _) = args.unpack();
        let quit = args.clone_quit();
        let mut event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let mut sys = WindowRendererSystem::new(self.args, &event_loop);

        event_loop.run_return(move |event, _, control_flow| match event {
            Event::RedrawRequested(_) => sys.update(&mut args),
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } => *control_flow = ControlFlow::ExitWithCode(0),
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(s) => sys.resize(s),

                _ => {
                    args.query::<()>();
                }
            },

            _ => {}
        });

        quit.store(true, Ordering::Release);
    }
}
