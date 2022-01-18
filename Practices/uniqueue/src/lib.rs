#[cfg(test)]
mod tests {
    use crate::Uniqueue;

    #[test]
    fn empty_uniqueue_test() {
        let qu = Uniqueue::<u8>::new();
        assert_eq!(qu.count(), 0);
        assert_eq!(qu.is_empty(), true);
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

    /// İçerideki eleman sayısını verir
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Kuyruğun boş olup olmadığını söyler
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
// clippy önerisiyle eklendi
impl<T: Clone> Default for Uniqueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
