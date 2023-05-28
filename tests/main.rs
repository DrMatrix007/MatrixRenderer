use std::f32::consts::PI;

use matrix_engine::{
    components::{component::ComponentCollection, resources::ResourceHolder},
    dispatchers::{
        component_group::ComponentGroup,
        context::{Context, SceneCreator},
        dispatcher::{DispatchedData, ReadStorage, WriteStorage},
        systems::{AsyncSystem, ExclusiveSystem},
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::{multi_threaded_scheduler::MultiThreadedScheduler, single_threaded_scheduler::SingleThreadScheduler},
};
use matrix_renderer::{
    math::{
        matrices::{IntoMatrix, Matrix, Vector3},
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
use rand::Rng;

use rayon::prelude::*;

struct CreateDataSystem;

impl AsyncSystem for CreateDataSystem {
    type Query = (
        WriteStorage<ComponentCollection<RenderObject>>,
        WriteStorage<ComponentCollection<Transform>>,
    );

    fn run(&mut self, ctx: &Context, (objects, transforms): &mut <Self as AsyncSystem>::Query) {
        let size_x = 250;
        let size_z = 250;

        let mut r = rand::thread_rng();

        for x in 0..size_x {
            for z in 0..size_z {
                let e = Entity::default();
                objects.get().insert(
                    e,
                    RenderObject::new(
                        Plain,
                        match (x) % 2 {
                            0 => "pic2.png".to_string(),
                            _ => "pic.png".to_string(),
                        },
                    ),
                );
                let mut t = Transform::identity();
                t.apply_position_diff([[x as f32, 0., -z as f32]].into());
                t.with_scale([[r.gen::<f32>(), r.gen::<f32>(), r.gen::<f32>()]].into_matrix());
                t.apply_rotation(
                    [[
                        r.gen_range(0.0..(2.0 * PI)),
                        r.gen_range(0.0..(2.0 * PI)),
                        r.gen_range(0.0..(2.0 * PI)),
                    ]]
                    .into(),
                );
                transforms.get().insert(e, t);

                ctx.destroy();
            }
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

        let speed = 40.0;
        let rotate_speed = PI / 4.0;

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
        delta = cam.camera().rotation.euler_into_rotation_matrix3() * delta * dt;
        let (a, b) = events.mouse_delta();
        self.theta += (a as f32) * dt * rotate_speed;
        self.phi += (b as f32) * dt * rotate_speed;
        *cam.camera_mut().rotation.y_mut() = self.theta;
        *cam.camera_mut().rotation.x_mut() = self.phi;
        cam.camera_mut().position += delta;
    }
}

struct RotateStuff;

impl AsyncSystem for RotateStuff {
    type Query = (
        WriteStorage<ComponentCollection<Transform>>,
        ReadStorage<ComponentCollection<RenderObject>>,
    );

    fn run(&mut self, _ctx: &Context, comps: &mut <Self as AsyncSystem>::Query) {
        let (ts,rs) = comps.get();
        ts.iter_mut().for_each(|x| {
            if rs.get(x.0).is_some() {
                x.1.apply_rotation(
                    Matrix::from([[
                        rand::random::<f32>() * 2.0 * PI,
                        rand::random::<f32>() * 2.0 * PI,
                        rand::random::<f32>() * 2.0 * PI,
                    ]]) / 100.,
                );
            }
        });
    }
}

fn main() {
    //std::env::set_var("RUST_BACKTRACE", "1");

    let engine = Engine::new(EngineArgs {
        fps: 144,
        scheduler: SingleThreadScheduler::new(),
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
        .add_exclusive_system(WindowSystem)
        .add_async_system(CameraPlayerSystem::new())
        .add_async_system(RotateStuff)
        ;
    engine.run(scene);
}
