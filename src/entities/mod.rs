pub mod blocks;
pub mod camera;

use crate::input::Input;
use crate::math::vector::Vector;
use crate::models::Model;
use glium::{glutin::event::Event, texture::Texture2d};
use std::rc::Rc;

pub struct Entity {
    pub model: Rc<Model>,
    pub texture: Option<Rc<Texture2d>>,
    pub position: Vector,
    pub rotation: Vector,
    pub color: Vector,
    pub scale: f32,
}

impl Entity {
    pub fn new(model: Rc<Model>) -> Self {
        Self {
            model,
            texture: None,
            position: Vector::default(),
            rotation: Vector::default(),
            color: Vector([1.0, 0.3, 0.3]),
            scale: 1.0,
        }
    }

    pub fn new_with_texture(model: Rc<Model>, texture: Rc<Texture2d>) -> Self {
        Self {
            model,
            texture: Some(texture),
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
