fn main() {
    let value = 12.34_f32;
    // Length tipini normal bir f32 tipi gibi kullanabiliriz.
    let l: Length = 23.45;
    let result = l + value;
    println!("{} + {} = {}", value, l, result);

    // Information'ı da normal bir String türü gibi ele alabiliriz.
    let info: Information = "Bir takım bilgiler...".to_string();
    println!("{}", info);
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
