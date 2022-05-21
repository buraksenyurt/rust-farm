use std::fmt::{Display, Formatter};

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

    println!("{:?}",get_product(ProductId(1234)).unwrap());
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
#[derive(Debug)]
pub struct ProductId(i32);
#[derive(Debug)]
pub struct Title(String);
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

pub fn get_product(id: ProductId) -> Result<Product> {
    Ok(Product {
        id,
        title: Title("Db'den gelen isim".to_string()),
        list_price: ListPrice(10.0),
    })
}
