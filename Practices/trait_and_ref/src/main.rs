fn main() {
    // Adım 1: Basit bir trait tanımlanır ve bir veri yapısı için uygulanıp kullanılır
    let rocky = Climber;
    rocky.teach();

    // Adım 2
    // Climber'ın referansını alalım
    let rocky_ref: &Climber = &rocky;
    // teach metodunu bu kez referans değeri üstünden çağıralım.
    // ki bunda da bir sorun olmayacaktır.
    rocky_ref.teach();

    // Adım 3: Trainer trait'ini implemente eden nesneleri kullanan fonksiyonu kullanalım
    // ki bunda da bir sorun olmayacaktır
    let the_rock = Climber;
    teach_to(the_rock);

    // Adım 4: Şimdiyse trait implemente eden nesne referansını kullanmaya çalışalım
    // E0277 the trait bound `&Climber: Trainer` is not satisfied hatası alınır.
    // the trait `Trainer` is not implemented for `&Climber`
    // Çözüm için Çözüm UGGLY'deki implementasyon kullanılabilir
    // ama kod tekrarını önlemen adına Çözüm GENERIC önerilir.
    let the_rock = Climber;
    let the_rock_ref: &Climber = &the_rock;
    teach_to(the_rock_ref);
}

// Adım 3 : Şimdi Trainer implementasyonu yapan nesneleri
// parametre olarak alarak alabilen bir fonksiyon tanımlayalım.
fn teach_to(t: impl Trainer) {
    t.teach();
}

// // Çözüm UGGLY: E0277 hatasının çözümünde kullanılabilecek tekniklerden birisi.
// // Bu her ne kadar E0277 İçin iyi bir çözüm gibi görünse de programa Climber gibi başka
// // bir veri yapısı eklediğimizde benzer uyarlamayı onun için de yapmamız gerekir.
// // İdeal çözüm GENERIC kısmındaki gibidir.
// impl Trainer for &Climber {
//     fn teach(&self) {
//         return (**self).teach(); // iki kez dereference söz konusu
//     }
// }

// Çözüm GENERIC: Blanket Implementation
// T tipi için Trainer uyarlaması söz konusu ise bunu &T için de yaparak,
// generic bir çözüm sunabiliriz.
// Bu durumda Çözüm UGGLY'ye gerek kalmaz.
// GENEL UYGULAMA NOTU (ANA FİKİR)
// Bir trait oluşturduğumuzda bunun &T, &mut T ve Box<T> için versiyonlarını hazırlamalıyız.
impl<T: Trainer> Trainer for &T {
    fn teach(&self) {
        return (**self).teach();
    }
}
// Tüm mutable Trainer referansları Trainer olabilmelidir
impl<T: Trainer> Trainer for &mut T {
    fn teach(&self) {
        return (**self).teach();
    }
}

// Box içine alınmış tüm Trainer'lar Trainer olabilmelidir
impl<T: Trainer> Trainer for Box<T> {
    fn teach(&self) {
        return (**self).teach();
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
