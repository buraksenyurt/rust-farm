// src klasörü altından musicbox modülünü kullanmak için eklenir
mod musicbox;

#[cfg(test)]
mod tests {
    // tests modülü içinden kullanmak istediğimiz diğer modülleri veya modül fonksiyonlarını
    // alttaki satırlarda olduğu gibi tanımlayabiliriz.
    use super::storage; // direkt modül adını vererek
    use crate::musicbox::{load_songs, play_song}; // modül içindeki fonksiyonları açıkça belirterek

    #[test]
    fn it_works() {
        load_songs("Metallica");
        play_song(2323);
        storage::move_to_cloud();
        storage::maintain::optimize(); // modül içindeki modüle :: ile ulaşarak
        assert!(1 == 1);
    }
}

mod storage {
    pub fn move_to_cloud() {
        println!("Yeni şarkılar bulut ortamına alınıyor");
    }

    // iç modül (nested module) tanımı
    pub mod maintain {
        pub fn optimize() {
            println!("Cihaz belleği optimize ediliyor");
        }
    }
}
