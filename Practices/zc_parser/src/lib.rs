/*
   Bu örnek sıfır maliyet kopyalama ile veri dönüştürme işini ele alıyor.
*/
use std::str::from_utf8;

pub struct Data<'a> {
    pub header: u8,
    //pub content: String,
    pub content: &'a str,
}

impl<'a> Data<'a> {
    // pub fn parse(input: &[u8]) -> Self {
    //     let header = input[0];
    //     // gelen byte dizisinde 1nci elemandan itibaren sonuna kadar olan kısım
    //     // unsigned 8 bit türünden bir vector'e alınır.
    //     let bytes = input[1..input.len()].to_vec();
    //     // bu vector'den utf8 formatında bir string dönüşümü yapılır
    //     // from_utf8, bytes isimli değişkenin sahipliği alır.
    //     // sıfır maliyett bir kopyalama ile parse işlemi gerçekleştirmek için
    //     // to_vec isimli fonksiyondan kurtulmak gerekir.
    //     let content = String::from_utf8(bytes).unwrap();
    //     Self { header, content }
    // }
    pub fn parse(input: &'a [u8]) -> Self {
        let header = input[0];
        let content = from_utf8(&input[1..input.len()]).unwrap();
        Self { header, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_input_parsed_to_data() {
        let sample: [u8; 12] = [
            255, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ' ' as u8, 'w' as u8,
            'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8,
        ];
        let data = Data::parse(&sample);
        assert_eq!("hello world", data.content);
    }

    // #[test]
    // fn should_build_error() {
    //     /*
    //     Bu test fonksiyonuna göre kod derlendiğinde borrow checker'a takılır ve hata alırız.
    //
    //     error[E0506]: cannot assign to `sample[_]` because it is borrowed
    //     --> src/lib.rs:54:9
    //     |
    //     52 |         let data = Data::parse(&sample);
    //     |                                ------- `sample[_]` is borrowed here
    //     53 |         assert_eq!("hello world", data.content);
    //     54 |         sample[1] = 'b' as u8;
    //     |         ^^^^^^^^^^^^^^^^^^^^^ `sample[_]` is assigned to here but it was already borrowed
    //     55 |         assert_eq!("bello world", data.content);
    //     |         --------------------------------------- borrow later used here
    //
    //      */
    //     let mut sample: [u8; 12] = [
    //         255, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ' ' as u8, 'w' as u8,
    //         'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8,
    //     ];
    //     let data = Data::parse(&sample);
    //     assert_eq!("hello world", data.content);
    //     sample[1] = 'b' as u8;
    //     assert_eq!("bello world", data.content);
    // }
}
