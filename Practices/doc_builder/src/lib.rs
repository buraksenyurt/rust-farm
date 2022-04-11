use std::fmt::{Display, Formatter};

// Bir şeyi çizme davranışını tanımlayan yeni bir trait nesnesi eklendi.
// Tek bir fonksiyonu var uygulandığı nesne ne ise onu referans olarak alıyor
pub trait Draw {
    fn draw(&self);
}

// Bu veri yapısı fatura veya benzeri bir evrakı temsil eden modelimiz olsun.
// En önemli özelliği kendi üstündeki bileşenleri taşıdığı Sections koleksiyonu.
// Çalışma zamanında Draw trait'ini uygulayan asıl tipler belirsiz olacağından
// Dynamic Dispatch yaklaşımına geçildi.
pub struct Document {
    pub sections: Vec<Box<dyn Draw>>,
}

impl Document {
    // sections içeriğine veri ekleme işini add fonksiyonuna verdik
    pub fn add(&mut self, section: Box<dyn Draw>) {
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

impl Title {
    pub fn new(text: String, sub_text: String) -> Self {
        Self { text, sub_text }
    }
}

// Title için Draw davranışını modelliyoruz(Sembolik olarak elbette)
impl Draw for Title {
    fn draw(&self) {
        println!("*****");
        println!("{}", self.text);
        println!("{}", self.sub_text);
        println!("*****\n");
    }
}

// Dokümanın alt kısmı için de Bottom isimli bir veri yapısı kurgulayalım
pub struct Bottom {
    pub summary: String,
}

impl Bottom {
    pub fn new(summary: String) -> Self {
        Self { summary }
    }
}

// Bottom veri yapısı içinde Draw davranışını yazıyoruz
impl Draw for Bottom {
    fn draw(&self) {
        println!("\n------\n{}\n-------", self.summary.to_uppercase());
    }
}

// Dokümana eklenebilecek ürün bilgilerini LineItems şeklinde bir veri yapısı olarak tutabiliriz.
// Faturanın orta kısımlarında alt alta kalemlerin yer aldığı, üstünde ürün adı, miktarı ve
// fiyatının olduğu bir grid düşünün.
#[derive(Default)]
pub struct LineItems {
    items: Vec<Product>,
}

impl LineItems {
    pub fn add(&mut self, p: Product) {
        self.items.push(p)
    }
}

// ve şimdi de LineItems'a Draw davranışını öğretelim
impl Draw for LineItems {
    fn draw(&self) {
        self.items.iter().for_each(|p| {
            println!("{}", p);
        })
    }
}

// Fatura dokümanında yer alabilecek ürün bilgileri Product struct'ı ile temsil edilebilir.
pub struct Product {
    pub id: u32,
    pub title: String,
    pub list_price: f32,
    pub quantity: u16,
}
impl Product {
    pub fn new(id: u32, title: String, list_price: f32, quantity: u16) -> Self {
        Self {
            id,
            title,
            list_price,
            quantity,
        }
    }
}
impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|t{}|t{}|t{}",
            self.id, self.title, self.quantity, self.list_price
        )
    }
}
