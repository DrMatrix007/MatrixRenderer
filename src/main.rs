use cameras::{Camera, CameraPrespective};
use drawables::{Square, SquareConfig};
use matrix_engine;
use pipelines::Renderer3D;
use renderer::Renderer;
use std::{env, f32::consts::PI};
use texture::TextureData;
use wgpu::{Color, FilterMode};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod cameras;
pub mod drawables;
pub mod math;
pub mod pipelines;
pub mod renderer;
pub mod texture;
pub mod vertex;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::<u32>::new(800, 800))
        .build(&event_loop)
        .unwrap();

    let c = Color {
        r: 0.4,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    };

    let mut renderer = Renderer::new(&window, c).await;

    let device = renderer.device();

    let queue = renderer.queue();

    let texture_data = TextureData::new(include_bytes!("./tree.png"), device, queue);

    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
    });

    let mut pipeline = Renderer3D::new_pipeline(
        device,
        renderer.config().format,
        Camera::Prespective(CameraPrespective {
            aspect: 1.0,
            eye: [0.0, 0.0, 1.0].into(),
            fovy_rad: 0.5 * PI,
            target: [0.0, 0.0, -1.0].into(),
            up: [0.0, 1.0, 0.0].into(),
            znear: 0.1,
            zfar: 100.0,
        }),
    );

    pipeline.add_drawable(Box::new(Square::new(&SquareConfig {
        device,
        pipeline: &pipeline,
        pos: &[-0.5, -0.5, -2.0],
        size: &[1.0, 1.0],
        sampler: &texture_sampler,
        view: texture_data.view(),
    })));

    pipeline.add_drawable(Box::new(Square::new(&SquareConfig {
        device,
        pipeline: &pipeline,
        pos: &[-0.25, -0.25, 0.0],
        size: &[0.5, 0.25],
        sampler: &texture_sampler,
        view: texture_data.view(),
    })));

    renderer.add_pipeline(Box::new(pipeline));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(a) => {
                renderer.resize(a);
                window.request_redraw();
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            renderer.render().unwrap();
        }

        _ => {}
    })
}
