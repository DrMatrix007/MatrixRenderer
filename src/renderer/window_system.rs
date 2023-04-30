use super::window::{MatrixWindow, MatrixWindowArgs};
use matrix_engine::dispatchers::context::ResourceHolderManager;
use matrix_engine::events::event_registry::EventRegistry;
use matrix_engine::{
    components::resources::ResourceHolder,
    dispatchers::{
        context::Context,
        systems::{AsyncSystem, ExclusiveSystem},
    },
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget};

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

    fn run(&mut self, ctx: &Context, (r, e): <Self as ExclusiveSystem>::Query<'_>) {
        let name = self.name.clone();
        let size = self.size;
        ctx.get_or_insert_resource_with(r, || {
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
    type Query<'a> = (&'a ResourceHolder<MatrixWindow>, &'a EventRegistry);

    fn run(&mut self, args: &Context, (window, events): <Self as AsyncSystem>::Query<'_>) {
        let Some(window) = window.get() else {
            return;
        };
        let events = events.get_window_events(window.id());

        if events.should_close() {
            args.quit();
        }
    }
}
