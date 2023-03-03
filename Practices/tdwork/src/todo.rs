#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Todo<'a> {
    pub id: i32,
    title: &'a str,
    completed: bool,
}

impl<'a> Todo<'a> {
    pub fn new(id: i32, title: &'a str) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn complete(&mut self) {
        self.completed = true
    }
}
