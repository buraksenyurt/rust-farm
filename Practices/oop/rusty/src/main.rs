use std::fmt::{Display, Formatter};

fn main() {
    // Robot nesnesi üstünde değişikliker yapacağımız için mutable olması gerekir
    // Sonrası C# tarafı ile oldukça benzerdir.
    let mut tars = Robot::new(String::from("TARS"), 80.0);
    println!("{}", tars);
    tars.load_fuel(10.0);
    println!("{}", tars);
    tars.walk(24.0, -50.9);
    println!("{}", tars);
}

// C# Örneğindeki gibi robotun anlık durumu için burada da bir enum kullanıyoruz.
// Tabii rust için enum bir veri yapısıdır. Zenginleştirilebilir. OnTheMove alanında olduğu gibi.
#[allow(dead_code)]
enum State {
    Online,
    OutOfService,
    OnTheMove(Location), // Bonus :) Rust enum veri yapısının zenginliğini kullandık. OnTheMove state'indeyken örneğin lokasyonunu da tutalım dedik.
    Destroyed,
}

struct Location {
    pub x: f32,
    pub y: f32,
}

// C# tarafındaki Robot sınıfı burada bir struct olarak tanımlanır.
// Malum Rust tarafında class diye bir kavram yok.
struct Robot {
    pub name: String,
    pub fuel_level: f32,
    state: State,
}

// C# tarafında Robot nesnesinin metotları(yapıcı metot dahil) sınıf tanım blokları içerisindedir.
// Rust fonksiyonel paradigmayı benimser ve aşağıdaki usülde ilerlenir.
impl Robot {
    // yapıcı metot karşılğı. Self ile çalışma zamanındaki Robot nesnesini ifade ederiz.
    pub fn new(name: String, fuel_level: f32) -> Self {
        Self {
            name,
            fuel_level,
            state: State::Online,
        }
    }
    // yakıt seviyesini artıran fonksiyon.
    // Tabii rust tarafında her değişken aksi belirtilmediği sürece immutable olduğundan,
    // self için &mut kullanılır.
    pub fn load_fuel(&mut self, amount: f32) {
        println!("{} litre yakıt yükleniyor.", amount);
        self.fuel_level += amount
    }

    // Rust örneğinde bonus olarak enum veri yapısını zenginleştirebileceğimizi göstermek istedim.
    pub fn walk(&mut self, x: f32, y: f32) {
        println!("Hareket halinde");
        self.state = State::OnTheMove(Location { x, y });
    }
}

// C# örneğinde ToString metodunu override edip Robot nesneleri için bu davranışı değiştirmiştik.
// Rust tarafında bunun için Display trait'ini Robot nesnesi için implemente edebiliriz.
impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. Yakıt {}. Durum {}",
            self.name, self.fuel_level, self.state
        )
    }
}

// Tabii Rust tarafında şöyle bir sorun olacaktır. Robot veri yapısının kullandığı
// State enum türü için de Display trait'ini uygulamamız gerekir.
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            State::Online => "Çalışıyor.".to_string(),
            State::Destroyed => "Yok edildi".to_string(),
            State::OutOfService => "Servis dışı".to_string(),
            State::OnTheMove(l) => format!("{},{} noktasında hareket halinde.", l.x, l.y),
        };
        write!(f, "{}", state)
    }
}
