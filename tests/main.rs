use std::f32::consts::PI;

use matrix_engine::{
    components::{component::ComponentCollection, resources::ResourceHolder},
    dispatchers::{
        context::{Context, SceneCreator},
        dispatcher::{ReadStorage, WriteStorage},
        systems::AsyncSystem,
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use matrix_renderer::{
    math::{
        matrices::{Matrix3, Vector3},
        vectors::Vector3D,
    },
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
    type Query = (
        WriteStorage<ResourceHolder<RendererResource>>,
        WriteStorage<ComponentCollection<RenderObject>>,
    );

    fn run(&mut self, ctx: &Context, (mut resource, mut objects): <Self as AsyncSystem>::Query) {
        if let Some(data) = resource.get_mut() {
            for _ in 0..1 {
                objects
                    .get_mut()
                    .insert(Entity::default(), RenderObject::new(data))
            }

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

    fn run(&mut self, ctx: &Context, (events, mut cam, window): <Self as AsyncSystem>::Query) {
        let (Some(cam),Some(window)) = (cam.get_mut(),window.get())  else {
            return;
        };
        let window_events = events.data().get_window_events(window.id());

        let mut delta = Vector3::zeros();

        let speed = 4.0;
        let rotate_speed = PI / 2.0;

        let dt = events.data().calculate_delta_time().as_secs_f32();

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

        let (a, b) = events.data().mouse_delta();
        self.theta += (a as f32) * dt * rotate_speed;
        self.phi += (b as f32) * dt * rotate_speed;

        let t = Matrix3::rotate_y(self.theta) * Matrix3::rotate_x(self.phi);

        let rotation = &t * Vector3::from([0., 0., -1.]);
        cam.camera_mut().dir = rotation;

        cam.camera_mut().eye += &t * &delta * dt;
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
        .add_async_system(CameraPlayerSystem::new());

    engine.run(scene);
}
