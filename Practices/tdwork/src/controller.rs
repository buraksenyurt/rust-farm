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
    pub fn get(&self, id: i32) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter().find(|t| t.id == id) {
            return Some(todo);
        }
        None
    }
    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
}
