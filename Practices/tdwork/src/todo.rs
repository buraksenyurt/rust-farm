use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Todo {
    pub id: u32,
    title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.id, self.title, self.completed)
    }
}
