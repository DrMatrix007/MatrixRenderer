use super::window::{MatrixWindow, MatrixWindowArgs};
use matrix_engine::dispatchers::context::ResourceHolderManager;
use matrix_engine::dispatchers::dispatcher::{
    ReadEventLoopWindowTarget, ReadStorage, WriteStorage,
};
use matrix_engine::events::event_registry::EventRegistry;
use matrix_engine::{
    components::resources::ResourceHolder,
    dispatchers::{
        context::Context,
        systems::{AsyncSystem, ExclusiveSystem},
    },
};
use winit::dpi::PhysicalSize;

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
    type Query = (
        WriteStorage<ResourceHolder<MatrixWindow>>,
        ReadEventLoopWindowTarget,
    );

    fn run(&mut self, ctx: &Context, (mut r, e): <Self as ExclusiveSystem>::Query) {
        let name = self.name.clone();
        let size = self.size;
        ctx.get_or_insert_resource_with(r.holder_mut(), || {
            MatrixWindow::new(MatrixWindowArgs {
                size,
                name,
                target: e.get(),
            })
        });
    }
}

pub struct WindowSystem;

impl AsyncSystem for WindowSystem {
    type Query = (
        ReadStorage<ResourceHolder<MatrixWindow>>,
        ReadStorage<EventRegistry>,
    );

    fn run(&mut self, args: &Context, (window, events): <Self as AsyncSystem>::Query) {
        let Some(window) = window.read() else {
            return;
        };
        let events = events.data().get_window_events(window.id());

        if events.should_close() {
            args.quit();
        }
    }
}
