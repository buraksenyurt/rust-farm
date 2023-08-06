#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub released: bool,
    pub summary: Option<String>,
}

impl Game {
    pub fn new(title: impl Into<String>) -> Game {
        Game {
            title: title.into(),
            released: false,
            summary: None,
        }
    }
}
