#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_actor(true);
        assert_eq!(result.kind, Kind::Human);
    }
}

// Actor isimli trait içerisinde move_to isimli bir fonksiyon bildirimi yer alıyor.
// uygulandığı türü alıp geriye literal string döndürmekte
trait Actor {
    fn move_to(&self) -> &str;
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
}

impl Actor for Tank {
    fn move_to(&self) -> &str {
        "Battle Master Tank hareket halinde"
    }
}

// Aşağıdaki gibi bir fonksiyon pekala düşünülebilir
// Yani belli bir koşula göre uygun Actor davranışını bezenmiş bir nesneyi döndürmek isteyebiliriz.
pub fn get_actor(condition: bool) -> Actor {
    if condition {
        Comrad { kind: Kind::Human }
    } else {
        Tank {
            kind: Kind::Vehicle,
        }
    }
}

pub enum Kind {
    Human,
    Vehicle,
}
