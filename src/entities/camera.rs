use crate::entities::Updatable;
use crate::input::{keyboard, Input};
use crate::math::vector::Vector;

const ANGLE: f32 = -55.0;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub rotation: Vector,
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector([0.0, 3.0, -3.0]),
            rotation: Vector([ANGLE.to_radians(), 0.0, 0.0]),
            speed: 0.01,
        }
    }
}

impl Updatable for Camera {
    fn update(&mut self, input: &Input) {
        if input.is_key_pressed(keyboard::W) {
            let z = self.speed / self.rotation.x().cos();
            self.position.set_z(self.position.z() + z);
            self.position.set_y(self.position.y() + z);
        }

        if input.is_key_pressed(keyboard::S) {
            let z = self.speed / self.rotation.x().cos();
            self.position.set_z(self.position.z() - z);
            self.position.set_y(self.position.y() - z);
        }

        if input.is_key_pressed(keyboard::A) {
            self.position.set_x(self.position.x() - self.speed);
        }

        if input.is_key_pressed(keyboard::D) {
            self.position.set_x(self.position.x() + self.speed);
        }
    }
}
