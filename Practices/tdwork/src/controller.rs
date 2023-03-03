use crate::todo::Todo;

pub struct Controller<'a> {
    todos: Vec<Todo<'a>>,
}

impl<'a> Default for Controller<'a> {
    fn default() -> Self {
        Self { todos: Vec::new() }
    }
}

impl<'a> Controller<'a> {
    pub fn add(&mut self, todo: Todo<'a>) {
        self.todos.push(todo)
    }
    pub fn get(&self, id: i32) -> Option<&'a Todo> {
        if let Some(todo) = self.todos.iter().find(|t| t.id == id) {
            return Some(todo);
        }
        None
    }
    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
    pub fn is_completed(&self, id: i32) -> bool {
        if let Some(todo) = self.todos.iter().find(|t| t.id == id) {
            return todo.completed;
        }
        false
    }
    pub fn complete(&mut self, id: i32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true
        }
    }
}
