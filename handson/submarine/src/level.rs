use std::fmt::{Display, Formatter};

pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Easy => write!(f, "Rokie"),
            Level::Medium => write!(f, "Senior"),
            Level::Hard => write!(f, "Master"),
        }
    }
}

impl From<&Level> for i32 {
    fn from(level: &Level) -> Self {
        match level {
            Level::Easy => 10,
            Level::Medium => 5,
            Level::Hard => 2,
        }
    }
}
