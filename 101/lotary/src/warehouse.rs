// Proje için kullanılan bir crate oluşturduk.
// Kendi dosyasında kendi modülüne sahip
pub mod wh {
    pub fn load_players(country: &str) {
        println!("{} ülkesinden yarışmacılar yükleniyor",country);
    }
}
