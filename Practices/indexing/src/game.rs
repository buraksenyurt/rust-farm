use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Game {
    pub id: u32,
    pub title: String,
    pub average_point: f32,
    pub release_year: u16,
    pub producer: String,
    pub platform: String,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}",
            self.id, self.title, self.average_point, self.producer, self.platform
        )
    }
}
