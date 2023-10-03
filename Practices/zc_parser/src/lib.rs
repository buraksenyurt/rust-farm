/*
   Bu örnek sıfır maliyet kopyalama ile veri dönüştürme işini ele alıyor.
*/
pub struct Data {
    pub header: u8,
    pub content: String,
}

impl Data {
    pub fn parse(input: &[u8]) -> Self {
        let header = input[0];
        // gelen byte dizisinde 1nci elemandan itibaren sonuna kadar olan kısım
        // unsigned 8 bit türünden bir vector'e alınır.
        let bytes = input[1..input.len()].to_vec();
        // bu vector'den utf8 formatında bir string dönüşümü yapılır
        // from_utf8, bytes isimli değişkenin sahipliği alır.
        // sıfır maliyett bir kopyalama ile parse işlemi gerçekleştirmek için
        // to_vec isimli fonksiyondan kurtulmak gerekir.
        let content = String::from_utf8(bytes).unwrap();
        Self { header, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_input_parsed_to_data() {
        const sample: [u8; 12] = [
            255, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ' ' as u8, 'w' as u8,
            'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8,
        ];
        let data = Data::parse(&sample);
        assert_eq!("hello world", data.content);
    }
}
