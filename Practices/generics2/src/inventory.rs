use crate::container::Container;

pub struct Inventory<T> {
    elements: Vec<T>,
}
impl<T> Container<T> for Inventory<T> {
    fn get(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
impl<T> Inventory<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Inventory { elements }
    }
}
