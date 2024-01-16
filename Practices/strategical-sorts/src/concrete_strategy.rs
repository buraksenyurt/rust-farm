use crate::strategy::SortStrategy;

pub struct Bubble;

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

pub struct Quick;

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

pub struct Insertion;

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
