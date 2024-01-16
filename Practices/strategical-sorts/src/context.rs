use crate::strategy::SortStrategy;
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct SortingMaster<T, S>
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
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            _phantom_data: PhantomData,
        }
    }
    pub fn sort(&self, data: &mut Vec<T>) {
        self.strategy.sort(data);
    }
}
