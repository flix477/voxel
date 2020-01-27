use crate::math::vector::Vector;
use core::ops::Mul;

#[derive(Default, Debug, Clone)]
pub struct Matrix(pub [[f32; 4]; 4]);

impl Matrix {
    pub fn transformation(translation: &Vector, rotation: &Vector, scale: f32) -> Self {
        let translation = get_translation_matrix(translation.x(), translation.y(), translation.z());
        let rotation = get_rotation_matrix(rotation.x(), rotation.y(), rotation.z());
        let scale = get_scale_matrix(scale);

        translation * rotation * scale
    }

    pub fn camera_matrix(translation: &Vector, rotation: &Vector) -> Self {
        let rotation = get_rotation_matrix(rotation.x(), rotation.y(), rotation.z());
        let translation =
            get_translation_matrix(-translation.x(), -translation.y(), -translation.z());

        rotation * translation
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.0[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self.0[i][j] = value
    }

    pub fn take(self) -> [[f32; 4]; 4] {
        self.0
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::default();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get(i, k) * rhs.get(k, j);
                }
                result.set(i, j, sum);
            }
        }

        result
    }
}

fn get_translation_matrix(x: f32, y: f32, z: f32) -> Matrix {
    Matrix([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn get_rotation_matrix(x: f32, y: f32, z: f32) -> Matrix {
    let (x_cos, x_sin) = (x.cos(), x.sin());
    let (y_cos, y_sin) = (y.cos(), y.sin());
    let (z_cos, z_sin) = (z.cos(), z.sin());

    Matrix([
        [
            y_cos * z_cos,
            -x_cos * z_sin + x_sin * y_sin * z_cos,
            x_sin * z_sin + x_cos * y_sin * z_cos,
            0.0,
        ],
        [
            y_cos * z_sin,
            x_cos * z_cos + x_sin * y_sin * z_sin,
            -x_sin * z_cos + x_cos * y_sin * z_sin,
            0.0,
        ],
        [-y_sin, x_sin * y_cos, x_cos * y_cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn get_scale_matrix(scale: f32) -> Matrix {
    Matrix([
        [scale, 0.0, 0.0, 0.0],
        [0.0, scale, 0.0, 0.0],
        [0.0, 0.0, scale, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn get_projection_matrix(
    fov: f32,
    // distance of closest visible object
    near_plane: f32,
    // distance of farthest visible object
    far_plane: f32,
    aspect_ratio: f32,
) -> Matrix {
    let y_scale = 1.0 / (fov / 2.0).tan();
    let x_scale = y_scale / aspect_ratio;
    let zp = far_plane + near_plane;
    let zm = far_plane - near_plane;

    Matrix([
        [x_scale, 0.0, 0.0, 0.0],
        [0.0, y_scale, 0.0, 0.0],
        [0.0, 0.0, -zp / zm, -(2.0 * far_plane * near_plane) / zm],
        [0.0, 0.0, -1.0, 0.0],
    ])
}
