use std::fmt::{Display, Formatter};

fn main() {
    let mut tars = Robot::new(String::from("TARS"), 80.0);
    // Tabii ortak özellikleri vehicle alanında tutuyoruz ve onun Display özelliğini kullanmalıyız.
    // Bu nedenle tars.vehicle şeklinde bir kullanım söz konusu.
    println!("{}", tars.vehicle);
    tars.vehicle.load_fuel(10.0);
    println!("{}", tars.vehicle);
    tars.walk(24.0, -50.9);
    println!("{}", tars.vehicle);

    let mut u12 = Submarine::new(String::from("u12"), 1400.10);
    println!("{}", u12.vehicle);
    u12.vehicle.load_fuel(100.90);
    println!("{}", u12.vehicle);
    u12.dive(800);
    println!("{}", u12.vehicle);
}

// C# Örneğindeki gibi robotun anlık durumu için burada da bir enum kullanıyoruz.
// Tabii rust için enum bir veri yapısıdır. Zenginleştirilebilir. OnTheMove ve Dive alanlarında olduğu gibi.
#[allow(dead_code)]
enum State {
    Online,
    OutOfService,
    OnTheMove(Location), // Bonus :) Rust enum veri yapısının zenginliğini kullandık. OnTheMove state'indeyken örneğin lokasyonunu da tutalım dedik.
    Dive(i32),           // Bonus
    Destroyed,
}

struct Location {
    pub x: f32,
    pub y: f32,
}
struct Vehicle {
    pub name: String,
    pub fuel_level: f32,
    state: State,
}
impl Vehicle {
    pub fn new(name: String, fuel_level: f32) -> Self {
        Self {
            name,
            fuel_level,
            state: State::Online,
        }
    }
}

// Hem Robot hem de Submarine için ortak olan load_fuel fonksiyonelliğini bir trait olarak tanımladık
trait Fuel {
    fn load_fuel(&mut self, amount: f32);
}

// Tanımladığımız trait'i Vehicle tipi için uyguladık.
impl Fuel for Vehicle {
    fn load_fuel(&mut self, amount: f32) {
        println!("{} litre yakıt yükleniyor.", amount);
        self.fuel_level += amount
    }
}

impl Display for Vehicle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. Yakıt {}. Durum {}",
            self.name, self.fuel_level, self.state
        )
    }
}

// Robot veri yapısı Vehicle türünden bir özellik barındırıyor.
// Bu Submarine için de uygulanıyor. Composition yaptığımızı düşünebiliriz.
struct Robot {
    pub vehicle: Vehicle,
}

impl Robot {
    // C# tarafındaki base constructor'ı çağırma işlevselliğini uygulamaya çalıştık diyebiliriz.
    // Aslında Robot'un içerdiği Vehicle nesnesini örnekleyip onu taşıyan bir Robot değişkeni dönüyoruz.
    pub fn new(name: String, fuel_level: f32) -> Self {
        Self {
            vehicle: Vehicle::new(name, fuel_level),
        }
    }

    // Bu robota has bir fonksiyon.
    pub fn walk(&mut self, x: f32, y: f32) {
        println!("Hareket halinde");
        self.vehicle.state = State::OnTheMove(Location { x, y });
    }
}

struct Submarine {
    pub vehicle: Vehicle,
}

impl Submarine {
    pub fn new(name: String, fuel_level: f32) -> Self {
        Self {
            vehicle: Vehicle::new(name, fuel_level),
        }
    }

    // Submarine'e özgün fonksiyon
    pub fn dive(&mut self, depth: i32) {
        println!("{} metreye dalıyor", depth);
        self.vehicle.state = State::Dive(depth);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            State::Online => "Çalışıyor.".to_string(),
            State::Destroyed => "Yok edildi".to_string(),
            State::OutOfService => "Servis dışı".to_string(),
            State::OnTheMove(l) => format!("{},{} noktasında hareket halinde.", l.x, l.y),
            State::Dive(m) => format!("{} metreye iniyor.", m),
        };
        write!(f, "{}", state)
    }
}
