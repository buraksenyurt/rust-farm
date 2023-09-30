/*
   Akıllı işaretçilerden(Smart Pointers) birisi olan Rc<T>
   aynı verinin birden fazla sahibi olabilmesine yardımcı olur.
   Ancak bu sahiplik tamamen immutable'dır.

   Diğer yandan Rc<T> ile RefCell<T> işaretçilerini bir arada kullanarak
   birden fazla sahibi olan veriyi alıp değiştirebiliriz.
   Örnek bu ikilinin kullanımını ele almaktadır.

   Bu ikilinin kullanımında dikkat edilmesi gereki. Aşağıdaki örnek kod parçasını
   göz önüne alalım.
*/
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let movies = Rc::new(RefCell::new(vec![
        Movie::new(1, "Con Vik Chapter II".to_string(), 6.7),
        Movie::new(2, "Konstantine".to_string(), 7.1),
        Movie::new(3, "Şavnşenk Redempşın".to_string(), 9.1),
    ]));
    let mut movies_ref = movies.borrow_mut();
    // Üst satırda mutable bir ödünç alma söz konusu. Bunun üstüne alt satırda mutable olarak
    // tekrar ödünç almaya çalışıyoruz. Aynı scope içinde iki mutable sahiplik olamayacaktır.
    // Dikkat edilmesi gereken RefCell'in bunu çalışma zamanı için yorumlatacak olması.
    // Build ettiğimizde örnek hata üretmez.
    // Çalışma zamanında ise
    // thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:26:12
    // şeklinde bir hata alırız.
    // movies.borrow_mut().push(Movie::new(4, "Star Wars I".to_string(), 8.5));

    // Burada movies'in değiştirilebilir bir sahibini alıp işlem yapabiliriz.
    // Yani movies_ref'i mutable olarak RefCell üstünden alıp üzerinde değişiklik yapılabilir.

    println!("{:#?}\n", *movies_ref);
    movies_ref.push(Movie::new(4, "Star Wars I".to_string(), 8.5));
    println!("{:#?}", movies_ref);
}

#[derive(Debug)]
struct Movie {
    id: i32,
    name: String,
    rate: f32,
}
impl Movie {
    fn new(id: i32, name: String, rate: f32) -> Self {
        Self { id, name, rate }
    }
}
