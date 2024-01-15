trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

struct Bubble;

impl SortStrategy for Bubble {
    fn sort(&self, data: &mut Vec<i32>) {
        let mut swapped;
        let len = data.len();

        for _ in 0..len {
            swapped = false;
            for j in 0..len - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

struct Quick;

impl SortStrategy for Quick {
    fn sort(&self, data: &mut Vec<i32>) {
        quick_sort(data, 0, data.len() as isize - 1);
    }
}

fn quick_sort(data: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let pivot = data[high as usize];
        let mut i = low - 1;
        for j in low..high {
            if data[j as usize] <= pivot {
                i += 1;
                data.swap(i as usize, j as usize);
            }
        }
        data.swap((i + 1) as usize, high as usize);
        let pi = i + 1;

        quick_sort(data, low, pi - 1);
        quick_sort(data, pi + 1, high);
    }
}

struct Insertion;

impl SortStrategy for Insertion {
    fn sort(&self, data: &mut Vec<i32>) {
        let len = data.len();
        for i in 1..len {
            let key = data[i];
            let mut j = i as isize - 1;
            while j >= 0 && data[j as usize] > key {
                data[(j + 1) as usize] = data[j as usize];
                j -= 1;
            }
            data[(j + 1) as usize] = key;
        }
    }
}

#[allow(dead_code)]
struct SortingMaster {
    strategy: Box<dyn SortStrategy>,
}

#[allow(dead_code)]
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
    fn quick_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let quick_sort = Quick;
        let sorter = SortingMaster::new(Box::new(quick_sort));
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn bubble_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let bubble_sort = Bubble;
        let sorter = SortingMaster::new(Box::new(bubble_sort));
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn insertion_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let insertion_sort = Insertion;
        let sorter = SortingMaster::new(Box::new(insertion_sort));
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn change_sort_strategy_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let insertion_sort = Insertion;
        let mut sorter = SortingMaster::new(Box::new(insertion_sort));
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);

        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let bubble_sort = Bubble;
        sorter.change(Box::new(bubble_sort));
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }
}
