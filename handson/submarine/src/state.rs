use bracket_lib::prelude::{BTerm, GameState};

/// Oyunun anlık durumuna ait görüntüsünü(snapshot) tutan nesnedir.
pub struct State {}

// Oyunda anlık durumu değiştirecek türden bir tick fonksiyonu olmalıdır.
// Oyun durumunu tutacağımız State veri yapısı için bu özelliği GameState trait'i ile State nesnemiz için uygulamaktayız.
impl GameState for State {
    // Oyun motoru ile oyun arasındaki köprüyü kuran fonksiyondur.
    fn tick(&mut self, ctx: &mut BTerm) {
        // Oyun penceresi ile iletişim için context'e ihtiyacımız vardır.
        // BTerm türünden olan ctx ile ekranı temizleyebilir
        // üzerine yeni nesneler konumlandırabilir ve başka etkileşimleri sağlayabiliriz.
        ctx.cls(); // Ekranı temizle
                   // Koordinat sistemine göre sol üst köşeye bir mesaj yazdırıyoruz.
                   // Sol üst köşe 0,0 konumu iken bracket-lib'e göre sağ alt köşe 79,49 konumudur.
        ctx.print(1, 1, "1,1 konumundan merhaba");
    }
}
