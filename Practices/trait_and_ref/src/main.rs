fn main() {
    // Bölüm 1: Basit bir trait tanımlanır ve bir veri yapısı için uygulanıp kullanılır
    let rocky = Climber;
    rocky.teach();

    // Bölüm 2
    // Climber'ın referansını alalım
    let rocky_ref: &Climber = &rocky;
    // teach metodunu bu kez referans değeri üstünden çağıralım.
    // ki bunda da bir sorun olmayacaktır.
    rocky_ref.teach();

    // Bölüm 3: Trainer trait'ini implemente eden nesneleri kullanan fonksiyonu kullanalım
    // ki bunda da bir sorun olmayacaktır
    let the_rock = Climber;
    teach_to(the_rock);

    // Bölüm 4: Şimdiyse trait implemente eden nesne referansını kullanmaya çalışalım
    // E0277 the trait bound `&Climber: Trainer` is not satisfied hatası alınır.
    // the trait `Trainer` is not implemented for `&Climber`
    // Çözüm için Bölüm 5teki implementasyon kullanılır.
    let the_rock = Climber;
    let the_rock_ref: &Climber = &the_rock;
    teach_to(the_rock_ref);
}

// Bölüm 3 : Şimdi Trainer implementasyonu yapan nesneleri
// parametre olarak alarak alabilen bir fonksiyon tanımlayalım.
fn teach_to(t: impl Trainer) {
    t.teach();
}

// Bölüm 5: E0277 hatasının çözümünde kullanılabilecek tekniklerden birisi.
impl Trainer for &Climber {
    fn teach(&self) {
        return (**self).teach(); // iki kez dereference söz konusu
    }
}

trait Trainer {
    fn teach(&self);
}

struct Climber;

impl Trainer for Climber {
    fn teach(&self) {
        println!("Sana, nasıl dağa tırmanılır onu öğreteceğim.")
    }
}
