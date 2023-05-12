use std::{f32::consts::PI, thread, time::Duration};

use matrix_engine::{
    components::{component::ComponentCollection, resources::ResourceHolder},
    dispatchers::{
        context::{Context, SceneCreator},
        dispatcher::{DispatchedData, ReadStorage, WriteStorage},
        systems::AsyncSystem,
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use matrix_renderer::{
    math::{
        matrices::{Matrix3, Matrix4, Vector3},
        vectors::Vector3D,
    },
    pipelines::{structures::plain::Plain, transform::Transform},
    renderer::{
        camera::CameraResource,
        render_object::RenderObject,
        renderer_system::RendererSystem,
        window::MatrixWindow,
        window_system::{WindowCreatorSystem, WindowSystem},
    },
};

struct CreateDataSystem;

impl AsyncSystem for CreateDataSystem {
    type Query = (
        WriteStorage<ComponentCollection<RenderObject>>,
        WriteStorage<ComponentCollection<Transform>>,
    );

    fn run(&mut self, ctx: &Context, (objects, transforms): &mut <Self as AsyncSystem>::Query) {
        for _ in 0..1 {
            let e = Entity::default();
            objects
                .get()
                .insert(e, RenderObject::new(Plain, "pic.png".to_string()));
            transforms.get().insert(e, Transform::identity());

            ctx.destroy();
        }
    }
}

struct CameraPlayerSystem {
    theta: f32,
    phi: f32,
}

impl CameraPlayerSystem {
    fn new() -> Self {
        Self { phi: 0., theta: 0. }
    }
}

impl AsyncSystem for CameraPlayerSystem {
    type Query = (
        ReadStorage<EventRegistry>,
        WriteStorage<ResourceHolder<CameraResource>>,
        ReadStorage<ResourceHolder<MatrixWindow>>,
    );

    fn run(&mut self, ctx: &Context, (events, cam, window): &mut <Self as AsyncSystem>::Query) {
        let (Some(cam),Some(window)) = (cam.get(),window.get())  else {
            return;
        };
        let events = events.get();
        let window_events = events.get_window_events(window.id());

        let mut delta = Vector3::<f32>::zeros();

        let speed = 4.0;
        let rotate_speed = PI / 2.0;

        let dt = events.calculate_delta_time().as_secs_f32();

        if window_events.is_pressed(winit::event::VirtualKeyCode::A) {
            *delta.x_mut() -= speed;
        }
        if window_events.is_pressed(winit::event::VirtualKeyCode::D) {
            *delta.x_mut() += speed;
        }
        if window_events.is_pressed(winit::event::VirtualKeyCode::W) {
            *delta.z_mut() -= speed;
        }
        if window_events.is_pressed(winit::event::VirtualKeyCode::S) {
            *delta.z_mut() += speed;
        }
        if window_events.is_pressed(winit::event::VirtualKeyCode::Space) {
            *delta.y_mut() += speed;
        }
        if window_events.is_pressed(winit::event::VirtualKeyCode::C) {
            *delta.y_mut() -= speed;
        }

        if window_events.is_pressed(winit::event::VirtualKeyCode::Escape) {
            ctx.quit();
        }

        let (a, b) = events.mouse_delta();
        self.theta += (a as f32) * dt * rotate_speed;
        self.phi += (b as f32) * dt * rotate_speed;
        // *cam.transform_mut().rotate.x_mut() = self.theta;
        // *cam.transform_mut().rotate.y_mut() = self.phi;
        // println!("{}", cam.transform_mut());
    }
}

fn main() {
    //std::env::set_var("RUST_BACKTRACE", "1");

    let engine = Engine::new(EngineArgs {
        fps: 144,
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
        .add_async_system(CameraPlayerSystem::new());

    engine.run(scene);
}
