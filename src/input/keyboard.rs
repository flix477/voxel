use glium::glutin::event::{ElementState, Event, ScanCode, WindowEvent};
use std::collections::hash_set::HashSet;

pub const W: ScanCode = 13;
pub const A: ScanCode = 0;
pub const S: ScanCode = 1;
pub const D: ScanCode = 2;

#[derive(Default)]
pub struct Keyboard {
    held_keys: HashSet<ScanCode>,
}

impl Keyboard {
    pub fn on_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if input.state == ElementState::Pressed {
                    self.held_keys.insert(input.scancode);
                } else {
                    self.held_keys.remove(&input.scancode);
                }
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, scancode: ScanCode) -> bool {
        self.held_keys.contains(&scancode)
    }
}
