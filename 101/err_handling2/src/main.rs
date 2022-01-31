use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum ProcessError {
    Danger,
}
impl Error for ProcessError {}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Materyalde element ihlali")
    }
}

fn process(material: String) -> Result<bool, ProcessError> {
    if material.to_lowercase().contains("nitro") {
        Err(ProcessError::Danger)
    } else {
        Ok(true)
    }
}

/*
   Öncelikle process isimli fonksiyona odaklanalım. Gelen içerikte nitro kelimesi geçiyorsa,
   bir ProcessError nesnesini Error olarak döndürmekte. Aksi takdirde işlerin yolunda gittiğine
   karar verip true dönüyor.

   Bu fonksiyon Result<T,Err> döndüğünden çağırdığımız yerde kontrol atlına almak isteyeceğiz.
   İlk akla gelen yol aşağıdaki gibi pattern matching kullanmak olacaktır. Err bloğunda,
   dikkat edileceği üzere hata yakalanır ve yine de kasıtlı olarak panic oluşturlur.
*/

fn main() {
    // let result = process(String::from("3 Karbon 1 nitro gliserin 2 azot"));
    // match result {
    //     Ok(r) => println!("İşlem sonucu {}", r),
    //     Err(e) => {
    //         println!("{:?}", e.to_string());
    //         panic!("Dikkat nitro");
    //     }
    // }

    /*
        Yukarıdaki gibi Result dönüşlerinde Err kontrolü yapıp panic oluşmasını istiyorsak,
        işi biraz daha kısaltmak mümkün. unwrap ve expect fonksiyonlarını bu amaçla kullanabiliriz.
        unwrap'ta eğer işlem başarılı ise doğrudan sonuca ulaşırız. Hata varsa'da panic oluşturulur.

        unwrap yerine expect tercih edersek yine hata durumlarında panik oluşmasını sağlarız ama
        bu sefer ek bir bilgi mesajı da verebiliriz.
    */
    // let result = process(String::from("2 azot 1 karbon 3 oksijen")).unwrap();
    // println!("İşlem sonucu {:?}", result);
    //
    // let result = process(String::from("2 azot 1 nitro")).unwrap();
    // println!("İşlem sonucu {:?}", result);

    let result = process(String::from("2 azot 1 karbon 3 oksijen")).expect("nitro var, kaçın!!!");
    println!("İşlem sonucu {:?}", result);

    let result = process(String::from("2 azot 1 nitro")).expect("nitro var, kaçın!!!");
    println!("İşlem sonucu {:?}", result);

    println!("Program devam ediyor");
}
