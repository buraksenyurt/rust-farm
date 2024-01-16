pub trait SortStrategy<T> {
    fn sort(&self, data: &mut Vec<T>);
}
