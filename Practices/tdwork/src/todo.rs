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
    pub fn format(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.completed)
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} ({})",
            self.id,
            self.title,
            match self.completed {
                true => {
                    "completed"
                }
                false => {
                    "not completed"
                }
            }
        )
    }
}
