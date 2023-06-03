use super::window::{MatrixWindow, MatrixWindowArgs};
use matrix_engine::dispatchers::context::ResourceHolderManager;

use matrix_engine::dispatchers::dispatcher::events::Events;
use matrix_engine::dispatchers::dispatcher::resources::{ReadResource, WriteResource};
use matrix_engine::dispatchers::dispatcher::{DispatchedData, ReadEventLoopWindowTarget};

use matrix_engine::{
    dispatchers::{
        context::Context,
        systems::{ExclusiveSystem},
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
    type Query = (WriteResource<MatrixWindow>, ReadEventLoopWindowTarget);

    fn run(&mut self, ctx: &Context, (r, e): &mut <Self as ExclusiveSystem>::Query) {
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

impl ExclusiveSystem for WindowSystem {
    type Query = (ReadResource<MatrixWindow>, Events);

    fn run(&mut self, args: &Context, (window, events): &mut <Self as ExclusiveSystem>::Query) {
        let Some(window) = window.get() else {
            return;
        };
        let events = events.get().get_window_events(window.id());

        if events.should_close() {
            args.quit();
        }
    }
}
