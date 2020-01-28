use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};

pub struct DisplayManager {
    display: Display,
}

impl DisplayManager {
    pub fn new(events_loop: &EventLoop<()>) -> Result<Self, DisplayCreationError> {
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new().with_depth_buffer(24);
        let display = Display::new(wb, cb, events_loop)?;

        Ok(Self { display })
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }
}
