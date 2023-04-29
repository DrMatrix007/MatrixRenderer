use matrix_engine::{
    engine::{Engine, EngineArgs},
    scene::Scene,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use matrix_renderer::renderer::{
    renderer_system::RendererSystem, window_system::{WindowCreatorSystem, WindowSystem},
};

fn main() {
    let engine = Engine::new(EngineArgs {
        fps: 144,
        resources: None,
        scene: Scene::default()
            .with_startup_exclusive_system(WindowCreatorSystem::new(
                "nice".to_owned(),
                (1000, 500).into(),
            ))
            .with_async_system(RendererSystem).with_async_system(WindowSystem),
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
    });

    engine.run()
}
