use crate::repository::read_db;
use crate::todo::Todo;

pub struct Controller {
    todos: Vec<Todo>,
}

impl Default for Controller {
    fn default() -> Self {
        Self { todos: read_db() }
    }
}

impl Controller {
    pub fn add(&mut self, title: String) -> u32 {
        let new_id = self.create_id();
        let todo = Todo::new(new_id, title);
        self.todos.push(todo);
        new_id
    }
    pub fn create_id(&self) -> u32 {
        if let Some(v) = self.todos.iter().map(|t| t.id).max() {
            return v + 1;
        }
        1
    }
    pub fn get(&self, id: u32) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter().find(|t| t.id == id) {
            return Some(todo);
        }
        None
    }
    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
    pub fn is_completed(&self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter().find(|t| t.id == id) {
            return todo.completed;
        }
        false
    }
    pub fn complete(&mut self, id: u32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true
        }
    }
    pub fn delete(&mut self, id: u32) {
        if let Some(idx) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(idx);
        }
    }
}
