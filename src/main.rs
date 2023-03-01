use std::env;

use drawable::{Drawable2D, Square};
use pipelines::{
    FragmentConfig, Pipeline, PipelineConfig, PrimitiveConfig, Renderer2D, VertexConfig,
};
use renderer::Renderer;
use texture::TextureData;
use vertex::Vertex;
use wgpu::{
    include_wgsl, BindGroupLayoutEntry, BindingType, Color, Face, FilterMode, FrontFace,
    PolygonMode, PrimitiveTopology, ShaderStages,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod drawable;
pub mod pipelines;
pub mod renderer;
pub mod texture;
pub mod vertex;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

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

    let mut pipeline =Renderer2D::new_pipeline(device, renderer.config().format);

    pipeline.add_drawable(Box::new(Square::new(
        device,
        &pipeline,
        texture_data.view(),
        &texture_sampler,
    )));

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
