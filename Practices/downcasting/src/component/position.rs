use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Position(f32, f32);
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(x, y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
