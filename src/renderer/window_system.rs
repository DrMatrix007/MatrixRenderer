use matrix_engine::{
    components::resources::ResourceHolder,
    dispatchers::systems::{AsyncSystem, ExclusiveSystem},
    events::Events,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget};

use super::window::{MatrixWindow, MatrixWindowArgs};

pub struct WindowCreatorSystem {
    name: String,
    size: PhysicalSize<u32>,
}

impl WindowCreatorSystem {
    pub fn new(name: String, size: PhysicalSize<u32>) -> Self {
        Self { name, size }
    }
}

impl ExclusiveSystem for WindowCreatorSystem {
    type Query<'a> = (
        &'a mut ResourceHolder<MatrixWindow>,
        &'a EventLoopWindowTarget<()>,
    );

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        (r, e): <Self as ExclusiveSystem>::Query<'_>,
    ) {
        let name = self.name.clone();
        let size = self.size;
        r.get_or_insert_with(|| {
            MatrixWindow::new(MatrixWindowArgs {
                size,
                name,
                target: e,
            })
        });
    }
}

pub struct WindowSystem;

impl AsyncSystem for WindowSystem {
    type Query<'a> = (&'a ResourceHolder<MatrixWindow>, &'a Events);

    fn run(
        &mut self,
        args: &matrix_engine::dispatchers::systems::SystemArgs,
        (window, events): <Self as AsyncSystem>::Query<'_>,
    ) {
        let Some(window) = window.get() else {
            return;
        };
        let events = events.get_window_events(window.id());

        if events.should_close() {
            args.stop();
        }
    }
}
