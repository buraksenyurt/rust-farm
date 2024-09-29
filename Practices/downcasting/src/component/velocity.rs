use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Velocity(f32, f32);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(x, y)
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
