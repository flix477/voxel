pub mod camera;

use crate::input::Input;
use crate::math::vector::Vector;
use crate::models::RawModel;
use glium::glutin::event::Event;
use std::rc::Rc;

pub struct Entity {
    pub model: Rc<RawModel>,
    pub position: Vector,
    pub rotation: Vector,
    pub color: Vector,
    pub scale: f32,
}

impl Entity {
    pub fn new(model: Rc<RawModel>) -> Self {
        Self {
            model,
            position: Vector::default(),
            rotation: Vector::default(),
            color: Vector([1.0, 0.3, 0.3]),
            scale: 1.0,
        }
    }
}

pub trait Updatable {
    fn on_event(&mut self, _: &Event<()>) {}
    fn update(&mut self, input: &Input);
}
