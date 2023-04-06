/*
 Kobay bir iş modülümüz olduğunu düşünelim.
 İçindeki fonksiyon harici bir servis çağrısı yapıyor ve servisten gelen cevaba göre
 süreci şekillendiriyor.

 Bunun testini yazarken servis sanki varmış gibi hareket edip fonksiyonu cover etmek istiyoruz.
 */
mod business {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(1 == 1)
    }
}
