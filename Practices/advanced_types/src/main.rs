use crate::Entity::{get_product, ProductId, Title};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    let value = 12.34_f32;
    // Length tipini normal bir f32 tipi gibi kullanabiliriz.
    let l: Length = 23.45;
    let result = l + value;
    println!("{} + {} = {}", value, l, result);

    // Information'ı da normal bir String türü gibi ele alabiliriz.
    let info: Information = "Bir takım bilgiler...".to_string();
    println!("{}", info);

    let pwd = Password("P@ssw0rd".to_string());
    println!("{}", pwd);

    // Product ile ilgili türleri Entity isimli bir modüle aldık.
    // Buna göre aşağıdaki kullanım varsayılan olarak bir hata verir.
    // cannot initialize a tuple struct which contains private fields
    // ProductId'nin i32 alanı private bir field olarak görünür.
    // Bu nedenle ProductId için esasında bir constructor yazmak gerekmiştir.
    let product_id = ProductId::new(1234);
    print!("Aranan ürün numarası {}", &product_id.as_i32());
    println!("{:#?}", get_product(product_id));
    //println!("{}",product_id.0); // Modüle içine alındığı için 0'a erişemeyiz.

    // From<i32> trait'ini implemente ettiğimiz için aşağıdaki kullanım mümkündür.
    let product_id = ProductId::from(4123);
    // Display trait'ini uyguladığımı için aşağıdaki kullanım da mümkündür.
    println!("Product ID {}", product_id);

    // Title tipi için FromStr,Display ve Deref trait'lerini uyguladığımız için
    // aşağıdaki kullanımlar geçerlidir
    let caption = Title::from_str("Rayzır Kulaküstü Mikrofonlu Kulaklık").unwrap();
    println!("Product title '{}'", caption);
    // Aşağıdaki kullanımda parametre olarak gelen değişken &Title'dan &str'a dönüştürülür.
    // Bunu deref implementasyonu ile sağlamaktayız.
    let caption = Title::from_str("Locitek kablosuz klavye").unwrap();
    use_product_title(&caption);
}

fn use_product_title(t: &str) {
    println!("{} ile ilgili bir şeyler yapılacak", t);
}

// Örnek Type Alias tanımlamaları

// Length tipi f32 türü için bir synonym'dir
type Length = f32;
// Benzer şekilde Information tipi de string için bir synonym'dir
type Information = String;

// type Alias'ların bir kullanım amacı uzun tanımları kısaltmak ve kod okunurluğunu artırmaktır.
// Örneğin aşağıdaki fonksiyona parametre olarak gelen Box tanımına bakalım.
// Bunu do_something_v2 'deki gibi type synonym'den yararlanarak da kullanabiliriz.
// Kod satır sayısı büyük projelerde bu yöntem epey yararlıdır nitekim hem tekrarlar azalır
// hem de kodun yönetilirliği kolaylaşır.
// Sadece fonksiyon parametresi olarak değil dönüş türlerinde de işe yarayacak bir tekniktir.
fn do_something(_func: Box<dyn Fn() + Send + 'static>) {
    println!("Do Something...");
}

type SendFunction = Box<dyn Fn() + Send + 'static>;

fn do_somehting_v2(_func: SendFunction) -> SendFunction {
    println!("Do Something V2...");
    todo!()
}

// Type Alias, Result<T,E> ie de sıklıkla kullanılır. Aşağıdaki trait'i ele alalım.
// Result<T,Error> türünü bir Type Alias olarak kullandırabiliriz.
// pub trait Transfer {
//     fn send(&mut self, x: f32, y: f32) -> Result<bool, std::io::Error>;
//     fn take(&mut self, size: i32) -> Result<&[u8], std::io::Error>;
// }

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Transfer {
    fn send(&mut self, x: f32, y: f32) -> Result<bool>;
    fn take(&mut self, size: i32) -> Result<&[u8]>;
}

/*
Şimdi şöyle bir senaryo düşünelim. Password bilgisini tutan bir veri yapısına ihtiyacımız var.
Bunun bir String olacağını öngörebiliriz. Lakin Display trait'ini uyguladığımızda password için
farklı bir davranış sergilenmesi gerekiyor. Mesela tüm harfleri * ile maskelemek gibi.
Bunun için bir tuple struct oluşturup tek bir alan kullanacak şekilde tasarlayabiliriz.
*/
pub struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*******")
    }
}

/*
Newtype deseni aslında tuple struct olarak adlandırılan enstrümanın özel bir uyarlamasıdır.
Konuyu daha iyi anlamak için aşağıdaki yapıyı ele alalım.
 */

// pub struct Product{
//     pub id:i32,
//     pub title:String,
//     pub list_price:f32
// }

/*
Pekala büyük çaplı bir projede aşağıdaki gibi bir fonksiyon yazılması söz konusu olabilir.
Burada parametre olarak gelen şey esasında nedir? Ne olmalıdır? Deneyimli bir programcı için
elbette verinin örneğin db'den getirilmesi adına id alanı olabilir.
Bunu garanti edebilmek için var olan primitive tipleri sarmalladığımız yeni tipleri ele alabiliriz.
 */
// pub fn get_product(product:String)->Result<Product>{
//     todo!()
// }

// Şöyle ki,
mod Entity {
    use std::fmt::{Display, Formatter};
    use std::ops::Deref;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct ProductId(i32);

    impl ProductId {
        pub fn new(id: i32) -> Self {
            Self(id)
        }

        pub fn as_i32(&self) -> i32 {
            self.0
        }
    }

    // i32 türünden bir değişkeni kullanarak ProductId türünden değişken örneklenmesi sağlanır
    impl From<i32> for ProductId {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    // Display trait'ini uygulayarak to_string fonksiyonelliği sağlanır
    impl Display for ProductId {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    pub struct Title(String);

    impl FromStr for Title {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Title(s.to_string()))
        }
    }

    impl Display for Title {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // String kullanan tuple veri yapılarına ait değişkenlerin
    // çeşitli fonksiyonlara referans olarak taşınmasında deref trait'i kullanılabilir
    impl Deref for Title {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct ListPrice(f32);

    // Product veri yapısı alanları üstteki tuple struct'lar ile donatılır
    // ProductId, Title ve ListPrice türlerinde istediğimiz kuralları işletebileceğimiz
    // implemantasyonlar yapabilir veya var olanları esnetebiliriz.
    #[derive(Debug)]
    pub struct Product {
        pub id: ProductId,
        pub title: Title,
        pub list_price: ListPrice,
    }

    pub fn get_product(id: ProductId) -> Product {
        Product {
            id,
            title: Title("Db'den gelen isim".to_string()),
            list_price: ListPrice(10.0),
        }
    }
}
