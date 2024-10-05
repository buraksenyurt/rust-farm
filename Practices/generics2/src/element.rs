/*
   Generic yapı kullanımına ait bir diğer örnek.
   İlk versiyonda Element veri modelinde String türü kullanılır.
   Sonraki sürümde ise generic tür kullanımına geçilir.
*/

use super::container::Container;

pub struct Element<T> {
    item: Option<T>,
}

impl<T> Container<T> for Element<T> {
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn push(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
impl<T> Element<T> {
    pub fn new(item: T) -> Self {
        Element { item: Some(item) }
    }
}

// Başlangıç
// pub struct Element {
//     item: Option<String>,
// }
// impl Element {
//     pub fn new(item: String) -> Self {
//         Element { item: Some(item) }
//     }
//
//     pub fn push(&mut self, item: String) {
//         self.item = Some(item);
//     }
//
//     pub fn get(&mut self) -> Option<String> {
//         self.item.take()
//     }
//
//     pub fn is_empty(&mut self) -> bool {
//         self.item.is_none()
//     }
// }
