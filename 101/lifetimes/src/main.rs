/*
   Örneğin ilk versiyonunda Component nesnesi bir Event nesnesinin referansını da kullanmakta.
   Yani button, bellekteki başka bir alanı da referans etmekte.

   Sorun şu ki, Event nesnesinin işaret ettiği bellek bölgesi bir sebepten geçersiz hale gelirse,
   Component nesnesi geçersiz bir bellek adresinin sahipliğini yapmaya başlar. Rust buna müsaade
   etmediğinden, Event nesne referansının gerektiği kadar yaşayacağının garanti edilmesini ister.
*/
fn main() {
    let click = Event {
        id: 1,
        name: String::from("clicked"),
    };
    let button = Component {
        id: 1000,
        name: String::from("btn_continue"),
        event: &click, // borrow ihlaline takılmamak için referansı ödünç vermeliyiz.
    };

    println!("{:#?}", click);
    println!("{:#?}", button);
}

// Aşağıdaki senaryoyu göz önüne alalım.
// Component veri yapısı kendi için Event türünden bir referans taşımaktadır.

#[allow(dead_code)]
#[derive(Debug)]
struct Event {
    id: i32,
    name: String,
}

// Yaşam süresini açıkça belirtiyoruz.
// Genelde 'a gibi bir notasyonla ifade edilmektedir.
// Buna göre bir Component nesnesi için gerekli yaşam ömrü ne ise event değişkeni de bu süre kadar yaşar.
#[allow(dead_code)]
#[derive(Debug)]
struct Component<'a> {
    id: i32,
    name: String,
    event: &'a Event,
}

// #[derive(Debug)]
// struct Component {
//     id: i32,
//     name: String,
//     event: &Event,
// }
