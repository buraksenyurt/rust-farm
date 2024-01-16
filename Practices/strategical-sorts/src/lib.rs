use std::marker::PhantomData;

trait SortStrategy<T> {
    fn sort(&self, data: &mut Vec<T>);
}

struct Bubble;

impl<T> SortStrategy<T> for Bubble
where
    T: PartialOrd,
{
    fn sort(&self, data: &mut Vec<T>) {
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

impl<T> SortStrategy<T> for Quick
where
    T: PartialOrd + Copy,
{
    fn sort(&self, data: &mut Vec<T>) {
        quick_sort(data, 0, data.len() as isize - 1);
    }
}

fn quick_sort<T: PartialOrd + Copy>(data: &mut Vec<T>, low: isize, high: isize) {
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

impl<T> SortStrategy<T> for Insertion
where
    T: PartialOrd + Copy,
{
    fn sort(&self, data: &mut Vec<T>) {
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
struct SortingMaster<T, S>
where
    S: SortStrategy<T>,
{
    strategy: S,
    _phantom_data: PhantomData<T>,
}

#[allow(dead_code)]
impl<T, S> SortingMaster<T, S>
where
    S: SortStrategy<T>,
{
    fn new(strategy: S) -> Self {
        Self {
            strategy,
            _phantom_data: PhantomData,
        }
    }
    fn sort(&self, data: &mut Vec<T>) {
        self.strategy.sort(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Quick);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn bubble_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Bubble);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn insertion_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Insertion);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }
}
