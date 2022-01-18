#[cfg(test)]
mod tests {
    use crate::Uniqueue;

    #[test]
    fn empty_uniqueue_test() {
        let qu = Uniqueue::<u8>::default();
        assert_eq!(qu.count(), 0);
        assert_eq!(qu.is_empty(), true);
    }

    #[test]
    fn enqueue_test() {
        let mut qu = Uniqueue::<i32>::new();
        let _ = qu.enq(23);
        let _ = qu.enq(32);
        let _ = qu.enq(11);
        assert_eq!(qu.count(), 3);
        match qu.enq(11) {
            Ok(result) => assert_eq!(result, true),
            Err(e) => assert_eq!(e, "Bu eleman zaten var"),
        }
    }
}

/// Benzersiz elemanlardan oluşan generic türden bir kuyruk koleksiyonu
#[derive(Debug)]
pub struct Uniqueue<T> {
    /// Elemanlar listesi
    pub items: Vec<T>,
}

impl<T: Clone + PartialEq> Uniqueue<T> {
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

    /// Kuyruğun sonuna yeni bir tane elemean ekler
    pub fn enq(&mut self, value: T) -> Result<bool, &str> {
        if self.items.contains(&value) {
            Err("Bu eleman zaten var")
        } else {
            self.items.push(value);
            Ok(true)
        }
    }
}
// clippy önerisiyle eklendi
impl<T: Clone> Default for Uniqueue<T> {
    fn default() -> Self {
        Uniqueue { items: Vec::new() }
    }
}
