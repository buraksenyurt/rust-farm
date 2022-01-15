use crate::Colors::Blue;

fn main() {
    let r = Colors::Red;
    println!("{:?}", r);

    let b = Blue; // yukarıda eklenen use bildirimi sebebiyle doğrudan da erişebiliriz
    println!("{:?}", b);

    hit_the_light(Colors::Green);

    let stable = Status::Stable(String::from("Tüm sistemler normal."));
    println!("{:?}", stable);

    let unstable = Status::NotStable(String::from("Motorlardaki rezonans çok yükse"));
    println!("{:?}", unstable);

    let overloaded = Status::Overload(32);
    println!("{:?}", overloaded);
}

fn hit_the_light(color: Colors) {
    if color == Colors::Green {
        println!("Yeşil arka plan fon yükleniyor.")
    }
}

#[derive(Debug, PartialEq)] // ?: için Debug, hit_the_light fonksiyonundaki == için PartialEq ekendi
enum Colors {
    Red,
    Green,
    Blue,
}

// enum değerlerine veri türleri bağlanabilir
#[derive(Debug)]
enum Status {
    Stable(String),
    NotStable(String),
    Overload(u8),
}
