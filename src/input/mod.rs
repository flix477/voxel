pub mod keyboard;

use glium::glutin::event::{Event, ScanCode};
use keyboard::Keyboard;

#[derive(Default)]
pub struct Input {
    keyboard: Keyboard,
}

impl Input {
    pub fn is_key_pressed(&self, key: ScanCode) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn on_event(&mut self, event: &Event<()>) {
        self.keyboard.on_event(event);
    }
}
