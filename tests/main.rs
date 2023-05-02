use matrix_engine::{
    components::{component::ComponentCollection, resources::ResourceHolder},
    dispatchers::{
        context::{Context, SceneCreator},
        systems::AsyncSystem,
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use matrix_renderer::{
    math::{matrices::Vector3, vectors::Vector3D},
    renderer::{
        camera::CameraResource,
        render_object::RenderObject,
        renderer_system::{RendererResource, RendererSystem},
        window::MatrixWindow,
        window_system::{WindowCreatorSystem, WindowSystem},
    },
};

struct CreateDataSystem;

impl AsyncSystem for CreateDataSystem {
    type Query<'a> = (
        &'a mut ResourceHolder<RendererResource>,
        &'a mut ComponentCollection<RenderObject>,
    );

    fn run(&mut self, ctx: &Context, (resource, objects): <Self as AsyncSystem>::Query<'_>) {
        if let Some(data) = resource.get_mut() {
            for _ in 0..1 {
                objects.insert(Entity::default(), RenderObject::new(data))
            }

            ctx.destroy();
        }
    }
}

struct CameraPlayerSystem;

impl AsyncSystem for CameraPlayerSystem {
    type Query<'a> = (
        &'a EventRegistry,
        &'a mut ResourceHolder<CameraResource>,
        &'a ResourceHolder<MatrixWindow>,
    );

    fn run(&mut self, ctx: &Context, (events, cam, window): <Self as AsyncSystem>::Query<'_>) {
        let (Some(cam),Some(window)) = (cam.get_mut(),window.get())  else {
            return;
        };
        let events = events.get_window_events(window.id());

        let mut delta = Vector3::zeros();

        if events.is_pressed(winit::event::VirtualKeyCode::A) {
            *delta.x_mut() += 0.1;
        }
        if events.is_pressed(winit::event::VirtualKeyCode::D) {
            *delta.x_mut() -= 0.1;
        }
        *cam.camera_mut().eye_mut() += &delta;
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
        .add_async_system(WindowSystem)
        .add_async_system(CameraPlayerSystem);

    engine.run(scene);
}
