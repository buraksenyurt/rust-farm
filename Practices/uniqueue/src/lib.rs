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

    #[test]
    fn dequeue_test() {
        let mut qu = Uniqueue::<f32>::new();
        let _ = qu.enq(3.14);
        let _ = qu.enq(2.22);
        let _ = qu.enq(1.25);

        let r = qu.deq().unwrap();
        assert_eq!(r, 3.14);
        let r = qu.deq().unwrap();
        assert_eq!(r, 2.22);
        let r = qu.deq().unwrap();
        assert_eq!(r, 1.25);

        match qu.deq() {
            Ok(_) => {}
            Err(e) => assert_eq!(e, "Kuyruk boş!"),
        }
    }

    #[test]
    fn peek_test() {
        let mut qu = Uniqueue::<f32>::new();
        let _ = qu.enq(3.14);
        let _ = qu.enq(2.22);
        let _ = qu.enq(1.25);
        let first_value = qu.peek();
        assert_eq!(first_value, Ok(&3.14));
        assert_eq!(qu.count(), 3);
    }
}

/// Benzersiz elemanlardan oluşan generic türden bir kuyruk koleksiyonu
#[derive(Debug)]
pub struct Uniqueue<T> {
    /// Elemanlar listesi
    pub items: Vec<T>,
}

// contains fonksiyonunun çalışması için PartialEq trait'ini uygulayan türler kullanılmalı.
// Ki kıyaslamalar yapılabilsin.
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

    /// Kuyruğun sonuna yeni bir tane elemean ekler.
    ///
    /// ## Errors
    ///
    /// Eklenmek istenen zaten kuyrukta ise hata döner
    pub fn enq(&mut self, value: T) -> Result<bool, &str> {
        if self.items.contains(&value) {
            Err("Bu eleman zaten var")
        } else {
            self.items.push(value);
            Ok(true)
        }
    }

    /// Kuyruğa eklenmiş elemanı çıkartır.
    ///
    /// ## Errors
    ///
    /// Kuyruk boşsa hata döner.
    pub fn deq(&mut self) -> Result<T, &str> {
        if !self.items.is_empty() {
            Ok(self.items.remove(0usize))
        } else {
            Err("Kuyruk boş!")
        }
    }

    /// İlk eklenen elemanı koleksiyondan çıkarmadan verir
    ///
    /// ## Errors
    ///
    /// Kuyrukta hiç eleman olmaması halinde hata döner.
    pub fn peek(&self) -> Result<&T, &str> {
        match self.items.first() {
            Some(value) => Ok(value),
            None => Err("Kuyruk boş!"),
        }
    }
}
// clippy önerisiyle eklendi
impl<T: Clone> Default for Uniqueue<T> {
    fn default() -> Self {
        Uniqueue { items: Vec::new() }
    }
}
