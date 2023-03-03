pub struct Todo {
    id: i32,
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
}
