/// Sunucu bilgilerini taşıyan veri yapısı.
pub struct Server {
    root: String,
    port: u16,
    alias: String,
}

impl Server {
    /// Yeni bir sunucu nesnesi örnekler.
    pub fn new(root: String, port: u16, alias: String) -> Self {
        Server { root, port, alias }
    }

    /// Sunucuyu dinleme modunda başlatır
    pub fn run(self) {
        /*
           Tipik bir HTTP sunucusu başlatıldığında sonsuz bir döngüde talep dinler.
           Dolayısıyla run fonksiyonu sonlandığında self ile ifade edilen ve
           sahiliği(ownership)'i alınan Server nesnesinin deallocate edilmesinde yarar vardır.
           Bu sebepten &self yerine self kullandık.
        */
        println!("Sunucu çalışıyor");
        //TODO Listener yazılacak
    }
}
