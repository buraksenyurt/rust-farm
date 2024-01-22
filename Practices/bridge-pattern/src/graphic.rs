use std::fmt::{Display, Formatter};

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.x, self.y, self.z)
    }
}
