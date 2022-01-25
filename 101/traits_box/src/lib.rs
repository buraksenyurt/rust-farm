#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_actor(true);
        assert_eq!(result.read_kind(), Kind::Human);

        let result = get_actor(false);
        assert_eq!(result.read_kind(), Kind::Vehicle);
    }
}

// Actor isimli trait içerisinde move_to isimli bir fonksiyon bildirimi yer alıyor.
// uygulandığı türü alıp geriye literal string döndürmekte
pub trait Actor {
    fn move_to(&self) -> &str;
    fn read_kind(&self) -> Kind;
}

// İki kobay struct
pub struct Comrad {
    pub kind: Kind,
}
pub struct Tank {
    pub kind: Kind,
}

// Actor trait'ini Comrad ve Tank yapılarına uyarlıyoruz.
impl Actor for Comrad {
    fn move_to(&self) -> &str {
        "Koşmaya başladım bile"
    }

    fn read_kind(&self) -> Kind {
        Kind::Human
    }
}

impl Actor for Tank {
    fn move_to(&self) -> &str {
        "Battle Master Tank hareket halinde"
    }

    fn read_kind(&self) -> Kind {
        Kind::Vehicle
    }
}

// Aşağıdaki gibi bir fonksiyon pekala düşünülebilir
// Yani belli bir koşula göre uygun Actor davranışını bezenmiş bir nesneyi döndürmek isteyebiliriz.
// pub fn get_actor(condition: bool) -> Actor {
//     if condition {
//         Comrad { kind: Kind::Human }
//     } else {
//         Tank {
//             kind: Kind::Vehicle,
//         }
//     }
// }

// Box işaretçisini kullandığımız yeni versiyon
// Comrad ve Tank değişkenleri için heap üstünde dinamik olarak yer ayrılır ve işaretçisi Box
// nesnesi tarafından ele alınır
pub fn get_actor(condition: bool) -> Box<dyn Actor> {
    if condition {
        Box::new(Comrad { kind: Kind::Human })
    } else {
        Box::new(Tank {
            kind: Kind::Vehicle,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Human,
    Vehicle,
}
