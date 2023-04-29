use matrix_engine::components::resources::Resource;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub struct MatrixWindowArgs<'a> {
    pub size: PhysicalSize<u32>,
    pub name: String,
    pub target: &'a EventLoopWindowTarget<()>,
}

pub struct MatrixWindow {
    window: winit::window::Window,
}

impl MatrixWindow {
    pub fn new(args: MatrixWindowArgs) -> Self {
        Self {
            window: WindowBuilder::new()
                .with_inner_size(args.size)
                .with_title(args.name)
                .build(args.target)
                .unwrap(),
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        let size = self.window.inner_size();
        PhysicalSize {
            width: size.width,
            height: size.height,
        }
    }

    pub fn raw(&self) -> &Window {
        &self.window
    }
    pub fn raw_mut(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn id(&self) -> WindowId {
        self.window.id()
    }
}

impl Resource for MatrixWindow {}
