use matrix_engine::{
    components::{component::ComponentCollection, resources::ResourceHolder},
    dispatchers::{
        context::{Context, SceneCreator},
        systems::{AsyncSystem, ExclusiveSystem},
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use matrix_renderer::renderer::{
    render_object::RenderObject,
    renderer_system::{RendererResource, RendererSystem},
    window_system::{WindowCreatorSystem, WindowSystem},
};

struct CreateDataSystem;

impl AsyncSystem for CreateDataSystem {
    type Query<'a> = (
        &'a mut ResourceHolder<RendererResource>,
        &'a mut ComponentCollection<RenderObject>,
    );

    fn run(&mut self, ctx: &Context, (resource, objects): <Self as AsyncSystem>::Query<'_>) {
        if let Some(data) = resource.get_mut() {
            for i in 0..1 {
                objects.insert(Entity::default(), RenderObject::new(data))
            }

            ctx.destroy();
        }
    }
}

fn main() {
    let engine = Engine::new(EngineArgs {
        fps: 144,
        resources: None,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
    });

    let ctx = engine.ctx();

    let mut scene = ctx.create_scene();

    scene
        .add_async_system(CreateDataSystem)
        .add_async_system(RendererSystem)
        .add_startup_exclusive_system(WindowCreatorSystem::new(
            "nice".to_owned(),
            (1000, 500).into(),
        ))
        .add_async_system(WindowSystem);

    engine.run(scene);
}
