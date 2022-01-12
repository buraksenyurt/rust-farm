/// Maket uçak yapımı sevenler için koleksiyonlarını yönetecekleri basit kütüphane.
///
/// # Bazı Yardımcı Bilgiler
///
/// Model uçak yapımı çok sevilen bir hobidir. Meşakkatli bir iştir ama sonuçları oldukça harikadır.
/// Yeni başlayanlar genelde 1:72 ölçekle çalışır. Az parçadan oluşan maketlerin bazıları için boya,
/// fırça, yapıştırıcı gibi unsurlar paketle birlikte gönderilir. İlk önce parçaların uygun şekilde
/// boyanması gerekir. Sonrasında plana uygun olarak yapıştırma işlemleri icra edilir. En son olarak
/// da logoların yapıştırılması işlemi uygulanır.
///
/// # İçerik
///
/// Kütüphanede yer alan temel enstrümanlar.
///
/// Bir maket modelinin temel bilgilerini taşır.
pub struct Model {
    /// Modelin başlığı. Örneğin Messerschmitt 109
    pub title: String,
    /// Model yapımının zorluk derecesi [Level]
    pub level: Level,
    /// Modelin parça sayısı
    pub part_count: u8,
    /// Güncel liste fiyatı
    pub list_price: f32,
}

#[derive(Debug)]
pub enum Level {
    /// Nispeten yapımı kolay olan seviye
    Easy,
    /// Artık güzel bir şeyler görmek isteyenlerin seviyesi
    Hard,
    /// Sınırları zorlayanların seviyesi
    Pro,
}

/// Uygulanabilecek en yüsek indirim oranı
pub const MAX_DISCOUNT_LEVEL: f32 = 10.99;

impl Model {
    /// Yeni bir model nesnesi oluşturmak için kullanılır.
    pub fn new(title: String, level: Level, part_count: u8, list_price: f32) -> Self {
        Model {
            title,
            level,
            part_count,
            list_price,
        }
    }

    /// Modelin fiyatına belirtilen miktarda indirim uygular
    pub fn apply_discount(&mut self, amount: f32) {
        if amount <= MAX_DISCOUNT_LEVEL {
            self.list_price -= amount
        } else {
            self.list_price -= MAX_DISCOUNT_LEVEL
        }
    }

    // cargo clippy sonrası aşağıdaki kullanım yerine Display trait'inin uyarlanması önerildi
    pub fn to_string(&self) -> String {
        format!(
            "{}. Zorluk {:?}.{} parça. Liste fiyatı {}",
            self.title, self.level, self.part_count, self.list_price
        )
    }
}
