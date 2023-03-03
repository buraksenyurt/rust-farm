use crate::todo::Todo;

pub struct Controller {
    todos: Vec<Todo>,
}

impl Default for Controller {
    fn default() -> Self {
        Self { todos: Vec::new() }
    }
}

impl Controller {
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }
}
