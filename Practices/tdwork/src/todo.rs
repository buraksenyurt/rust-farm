use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Todo<'a> {
    pub id: i32,
    title: &'a str,
    pub completed: bool,
}

impl<'a> Todo<'a> {
    pub fn new(id: i32, title: &'a str) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

impl<'a> Display for Todo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.id, self.title, self.completed)
    }
}
