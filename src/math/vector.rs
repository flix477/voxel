#[derive(Default, Clone, Copy, Debug)]
pub struct Vector(pub [f32; 3]);

impl Vector {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn set_x(&mut self, value: f32) {
        self.0[0] = value;
    }

    #[inline]
    pub fn set_y(&mut self, value: f32) {
        self.0[1] = value;
    }

    #[inline]
    pub fn set_z(&mut self, value: f32) {
        self.0[2] = value;
    }

    #[inline]
    pub fn take(self) -> [f32; 3] {
        self.0
    }
}
