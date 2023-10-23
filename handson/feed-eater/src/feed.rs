use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Feed<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

impl<'a> Feed<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        Self { title, url }
    }
}

impl<'a> Display for Feed<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.title, self.url)
    }
}
