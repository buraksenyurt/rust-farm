#[derive(Debug, PartialEq)]
pub struct Todo {
    pub id: i32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
