#[cfg(test)]
mod tests {
    use crate::Uniqueue;

    #[test]
    fn empty_uniqueue_test() {
        let qu = Uniqueue::<u8>::new();
        assert_eq!(qu.items.len(), 0);
    }
}

/// Benzersiz elemanlardan oluşan generic türden bir kuyruk koleksiyonu
#[derive(Debug)]
pub struct Uniqueue<T> {
    /// Elemanlar listesi
    pub items: Vec<T>,
}

impl<T: Clone> Uniqueue<T> {
    /// Boş bir Uniqueue oluşturur
    pub fn new() -> Uniqueue<T> {
        Uniqueue { items: Vec::new() }
    }
}
