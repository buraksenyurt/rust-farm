pub enum Level {
    Easy,
    Medium,
    Hard,
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
