use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent},
    Display,
};

pub struct DisplayManager {
    display: Display,
    should_close: bool,
}

impl DisplayManager {
    pub fn new(events_loop: &EventsLoop) -> Result<Self, DisplayCreationError> {
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, events_loop)?;

        Ok(Self {
            display,
            should_close: false,
        })
    }

    pub fn on_event(&mut self, event: &Event) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    self.should_close = true;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }
}
