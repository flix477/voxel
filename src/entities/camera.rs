use crate::entities::Updatable;
use crate::input::{keyboard, Input};
use crate::math::vector::Vector;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub rotation: Vector,
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector::default(),
            rotation: Vector::default(),
            speed: 0.01,
        }
    }
}

impl Updatable for Camera {
    fn update(&mut self, input: &Input) {
        if input.is_key_pressed(keyboard::W) {
            self.position.set_z(self.position.z() - self.speed);
            dbg!(&self);
        }

        if input.is_key_pressed(keyboard::S) {
            self.position.set_z(self.position.z() + self.speed);
            dbg!(&self);
        }
    }
}
