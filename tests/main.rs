use matrix_engine::engine::EngineBuilder;
use matrix_renderer::renderer::window_renderer_system::{WindowRendererRunner, WindowRendererSystemArgs};
use wgpu::Color;

fn main() {
    let engine = EngineBuilder::new()
        .with_fps(144)
        .with_registry_builder(|_reg| {})
        .with_runner(WindowRendererRunner::new(WindowRendererSystemArgs {
            size: (1000,500).into(),
            name: "Matrix Renderer".to_string(),
            color: Color::WHITE
        }))
        .build();

    engine.run();
}
