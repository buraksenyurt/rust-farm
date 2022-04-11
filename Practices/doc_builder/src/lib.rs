// Bir şeyi çizme davranışını tanımlayan yeni bir trait nesnesi eklendi.
// Tek bir fonksiyonu var uygulandığı nesne ne ise onu referans olarak alıyor
pub trait Draw {
    fn draw(&self);
}

// Bu veri yapısı fatura veya benzeri bir evrakı temsil eden modelimiz olsun.
// En önemli özelliği kendi üstündeki bileşenleri taşıdığı Sections koleksiyonu.
// Dikkat edileceği üzere bu ilk versiyonda sections kısmı Draw trait'ini uygulayan generic bir tip kullanıyor
// Yani Draw davranışını kullanan herkes bu listeye eklenebilir.
pub struct Document<T: Draw> {
    sections: Vec<T>,
}

impl<T> Document<T>
where
    T: Draw,
{
    // sections içeriğine veri ekleme işini add fonksiyonuna verdik
    pub fn add(&mut self, section: T) {
        self.sections.push(section)
    }
    // print fonksiyonu belgenin sections kısmındaki tüm nesneleri dolaşacak
    // ve her birinin Draw fonksiyonunu çağıracak.
    pub fn print(&self) {
        self.sections.iter().for_each(|m| m.draw())
    }
}

// Şimdi Draw işlevini uygulayan birkaç veri yapısı ekleyelim.
// Örneğin dokümanın başlık kısmı için Title isimli bir veri yapısı olabilir.
pub struct Title {
    pub text: String,
    pub sub_text: String,
}

impl Draw for Title {
    fn draw(&self) {
        println!("*****");
        println!("{}", self.text);
        println!("{}", self.sub_text);
        println!("*****\n");
    }
}

pub struct Bottom {
    pub summary: String,
}

impl Draw for Bottom {
    fn draw(&self) {
        println!("{}", self.summary.to_uppercase());
    }
}
