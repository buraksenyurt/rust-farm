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
    let result = process(String::from("3 Karbon 1 nitro gliserin 2 azot"));
    match result {
        Ok(r) => println!("İşlem sonucu {}", r),
        Err(e) => {
            println!("{:?}", e.to_string());
            panic!("Dikkat nitro");
        }
    }
    println!("Program devam ediyor");
}
