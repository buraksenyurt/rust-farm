use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Feed {
    pub title: String,
    pub url: String,
}

impl Feed {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

impl Display for Feed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.title, self.url)
    }
}
