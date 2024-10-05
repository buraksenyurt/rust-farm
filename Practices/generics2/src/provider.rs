use crate::container::Container;

pub struct Provider {}

/*
   Provider veri yapısı içerisinde Trait Bound kullanımı söz konusudur.
   T türünün bir Container olması beklenir.
   Farklı metotlarda farklı veri türleri ile öğe ekleme işlemleri ele alınabilir.
   add_string, String türünü kullanan Container trait'ini implemente eden nesnelerle çalışır.
   Benzer şekilde add_number'da Container trait'inin i32 için implemente etmiş nesnelerle çalışır.
*/
impl Provider {
    pub fn add_string<T: Container<String>>(c: &mut T, item: String) {
        c.push(item);
    }
    pub fn add_number<T: Container<i32>>(c: &mut T, item: i32) {
        c.push(item);
    }
}
