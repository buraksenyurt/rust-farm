use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::ops::{Add, Mul};
/*
   generic konusunu kısaca hatırlayalım.
*/

fn main() {
    let total = sum(4.14, 2.4);
    println!("Total: {}", total);

    let total = sum(4, 6);
    println!("Total: {}", total);

    let squared = square(4);
    println!("Squared result: {}", squared);

    let student_exam_point = Point { value: 85.50 };
    println!("Student exam result: {}", student_exam_point.value);

    let elo_point = Point { value: 1600 };
    println!("Chess Master's Elo score: {}", elo_point.value);

    println!("Elo Type Id {:?}", get_identity(elo_point));

    let objects: Vec<Box<dyn Component>> = vec![Box::new(Health(100)), Box::new(Damage(2))];
    some_system(objects);
}

/*
    Add trait'ini destekleyen (dolayısıyla) toplama operasyonunu destekleyen
    ve generic denince ilk akla gelen fonksiyonlardan birisi ile başlayabiliriz.

    operasyon olarak toplama yaptığımız için Add trait'i gerekmiştir.
*/
fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

/*
   Kendisini katlayan bir çarpma işlemi söz konusu olduğunda ise,
   Mul trait'ini kural olarak belirtmek yeterli olmaz.
   "Value used after being moved" hatası alınır.
   Bu nedenle Mul trait'i yanında Copy ve Clone trait'lerine de ihtiyaç duyulur
*/
fn square<T: Mul<Output = T> + Copy + Clone>(value: T) -> T {
    value * value
}

/*
   struct gibi veri yapılarında da generic türler kullanılabilir.
   Örneğin Point, Add, Copy ve Clone kabiliyetlerine sahip türleri value alanında taşıyabilir.
*/
struct Point<T: Add<Output = T> + Copy + Clone> {
    value: T,
}

/*
   Rust dünyasından her nesnenin bir type id değeri vardır.
   Any trait'i üstünden hareketle T türüne ait type id değerini alabileceğimiz generic
   bir fonksiyon aşağıdaki gibi yazılabilir.
*/
fn get_identity<T: Any>(object: T) -> TypeId {
    object.type_id()
    // TypeId::of::<T>() // Yukarıdaki ile aynı işi yapar
}

/*
   Aşağıdaki senaryoda kendi tanımladığımız Component trait'ini implemente eden türlerin
   parametre olarak geçildiği generic bir fonksiyon denemesi söz konusu.

   Rust dilinde generic kullanıldığında derleyici her tür için ayrı kod üretir.
   Lakin trait nesneleri dinamik olarak çözümlenir ve bir vector'de trait kullanılırsa
   farklı türleri derleyicinin yorumlayabilmesi için dyn keyword ele alınır.
   Nitekim tür boyutları derleme zamanında kesin olarak bilinemez. Bu bizi
   generic kullanılmayan ve Boxing ile nesne referanslarını yönettiğimiz diğer versiyona götürür.

*/
// fn some_system<T: Component + Debug>(objects: Vec<T>) {
//     for object in objects {
//         println!("{:?}", object);
//     }
// }

fn some_system(objects: Vec<Box<dyn Component>>) {
    for object in objects {
        println!("{:?}", object);
    }
}

/*
   Eğer Debug trait'inden türetme yapmazsak some_system fonksiyonundaki
   println! için şu hatayı alırız.
   `dyn Component` cannot be formatted using `{:?}` because it doesn't implement `Debug`
*/
trait Component: Debug {}

#[derive(Debug)]
struct Health(u32);
impl Component for Health {}

#[derive(Debug)]
struct Damage(u32);
impl Component for Damage {}
