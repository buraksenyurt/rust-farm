use std::fmt::{Display, Formatter};

fn main() {
    let mut tars = Robot::new(String::from("TARS"), 80.0);
    let mut u12 = Submarine::new(String::from("u12"), 1400.10);
    let mut alpha = Submarine::new(String::from("alpha"), 2000.0);
    // static dispatch
    // call_tools(tars);
    // call_tools(u12);
    // call_tools(alpha);

    // dynamic dispatch
    // call_tools_dynamic(&mut tars);
    // call_tools_dynamic(&mut u12);
    // call_tools_dynamic(&mut alpha);

    let mut abilities: Vec<&mut dyn Ability> = vec![];
    abilities.push(&mut tars);
    abilities.push(&mut u12);
    abilities.push(&mut alpha);

    for a in abilities {
        a.set_tools();
    }
}

// Static Dispatch
// Burada trait'in generic sürümüne başvurulur.
// Ability trait'ini uygulamış türler bu fonksiyona girebilir.
// Lakin Rust derleyicisi yukarıdaki call_tools çağrılarına göre her tip için aşağıdaki fonksiyonu yeniden yazıp derlenmiş koda gömer.
// fn call_tools<T: Ability>(mut a: T) {
//     a.set_tools();
// }

// Bir diğer alternatif yolda Dynamic Dispatch kullanımıdır.
// Özellikle library geliştiriyorsak Ability trait'ini asıl uygulayan tipi bilemeyebiliriz. Bunu runtime'da çözümlemek adına
// dyn anahtar kelimesinden yararlanırız.
fn call_tools_dynamic(a: &mut dyn Ability) {
    a.set_tools();
}

#[allow(dead_code)]
enum State {
    Online,
    OutOfService,
    OnTheMove(Location),
    Dive(i32),
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

// Araçların çeşitli alet ve edavatları yüklemesi için kodlayabilecekleri bir fonksiyon tanımladık.
trait Ability {
    fn set_tools(&mut self);
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

struct Robot {
    pub vehicle: Vehicle,
}

impl Robot {
    pub fn new(name: String, fuel_level: f32) -> Self {
        Self {
            vehicle: Vehicle::new(name, fuel_level),
        }
    }
}

// Ability trait'ini Robot türü için uyarladık
impl Ability for Robot {
    fn set_tools(&mut self) {
        println!(
            "{} için termal görüş sistemi, oksijen seviyesi ölçer yükleniyor.",
            self.vehicle.name
        );
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
}

// Ability trait'ini Submarine türü için uyarladık
impl Ability for Submarine {
    fn set_tools(&mut self) {
        println!(
            "{} için sonar, derinlik ölçer, ek batarya yükleniyor.",
            self.vehicle.name
        );
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
