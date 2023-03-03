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
    pub fn add(&mut self, todo: &'a Todo) {
        self.todos.push(*todo)
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
}
