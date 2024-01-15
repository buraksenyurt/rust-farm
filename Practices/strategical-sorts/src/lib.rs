pub fn add(left: usize, right: usize) -> usize {
    left + right
}

trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }
}

struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        todo!()
    }
}

struct SortingMaster {
    strategy: Box<dyn SortStrategy>,
}

impl SortingMaster {
    fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Self { strategy }
    }
    fn change(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy
    }
    fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
