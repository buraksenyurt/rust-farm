# C# Tarafında Class, Peki Ya Rust Tarafında ???

Rust'ın fonksiyonel paradigma'yı desteklemesi bir yana imperative özellikleri de benimsediğini söyleyebiliriz. Bu açıdan bakıldığında nesne yönelimli dillerde uygulanan bazı kavramların Rust tarafında da uygulanabiliyor olması beklenebilir. Bu klasörde iki örnek yer alıyor. Birisi Dotnet platformuna ait basit bir Console uygulaması. Adı sharpy. Diğeri ise rusty isimli ve Rust ile geliştirilen bir console uygulaması.

```shell
dotnet new console -n sharpy

cargo new rusty
```

Çalışmadaki amaç **OOP-Object Oriented Programming*** prensiplerinin Rust tarafında uygulanıp uygulanamayacağını görmek.

## Basit Nesne Tasarımı ve Override

İlk olarak dotnet tarafındaki console uygulamasını ele alalım. Bir oyundaki robotu temsile edecek veri yapısını kuvvetle muhtemel bir sınıf olarak tasarlarız.

```csharp
var tars = new Robot("TARS", 80);
Console.WriteLine(tars.ToString());
tars.LoadFuel(10);
Console.WriteLine(tars.ToString());
tars.Walk();
Console.WriteLine(tars.ToString());

public enum State
{
    Online,
    OutOfService,
    OnTheMove,
    Destroyed
}
class Robot
{
    public string Name { get; set; }
    public float FuelLevel { get; set; }
    private State State { get; set; }

    public Robot(string name, float fuel)
    {
        Name = name;
        FuelLevel = fuel;
        State = State.Online;
    }

    public void LoadFuel(float amount)
    {
        Console.WriteLine($"{amount} litre yakıt yükleniyor...");
        this.FuelLevel += amount;
    }

    public void Walk()
    {
        Console.WriteLine("Hareket halinde");
        this.State = State.OnTheMove;
    }

    public override string ToString()
    {
        return $"{this.Name}. Yakıt {this.FuelLevel}. Durum {this.State}";
    }
}
```

Yukarıdaki örnek şu an için aşağıdaki gibi çalışacaktır.

![../images/oop_1.png](../images/oop_1.png)

Robot sınıfının iki public ve bir de private özelliği var. Auto-Property olduklarından get ve set blokları dotnet tarafından tamamlanıyor. Parametreler ile overload edilmiş bir yapıcı metodu var. Ayrıca Walk ve LoadFuel isimli deneysel fonksiyonlara sahip. Object sınıfından gelen ToString metotu bu sınıf için yeniden yazıldı *(override)* Robotun o anki durumunu tutmak için State isimli bir Enum sabiti kullanıyoruz. Benzer bir modeli Rust tarafında inşa etmek istersek aşağıdaki gibi ilerleyebiliriz.

```rust
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
```

Rust örneğinin çalışma zamanı çıktısı da aşağıdaki gibi olacaktır.

![../images/oop_2.png](../images/oop_2.png)

## Kalıtım(Inheritance) Durumu

Gelelim nesne yönelimli programlamanın önemli kavramlarından birisi olan türetmeye. Bunu için dotnet tarafındaki uygulamamızı değiştirerek ilereyelim. Örneğin Robot haricinde Submarine şeklinde bir sınıfımız da olsun. Her ikisinin ortak özellik ve fonksiyonlarını bir üst sınıfta toplayalım. Vehicle tipik bir abstract sınıf rolünde. Robot ve Submarine sınıfları ortak özellikleri ve örneğin yapıcı metot için bu sınıfa geliyorlar.

```csharp
Robot tars = new Robot("TARS", 80);
Console.WriteLine(tars.ToString());
tars.LoadFuel(10);
Console.WriteLine(tars.ToString());
tars.Walk(23, -51);
Console.WriteLine(tars.ToString());

Submarine u12 = new Submarine("u12", 1200);
Console.WriteLine(u12.ToString());
u12.LoadFuel(10);
Console.WriteLine(u12.ToString());
u12.Dive(800);
Console.WriteLine(u12.ToString());

enum State
{
    Online,
    OutOfService,
    OnTheMove,
    Dive,
    Destroyed
}
class Vehicle
{
    public string Name { get; set; }
    public float FuelLevel { get; set; }
    protected State State { get; set; }

    public Vehicle(string name, float fuelLevel)
    {
        Name = name;
        FuelLevel = fuelLevel;
        State = State.Online;
    }
    public override string ToString()
    {
        return $"{this.Name}. Yakıt {this.FuelLevel}. Durum {this.State}";
    }
    public void LoadFuel(float amount)
    {
        Console.WriteLine($"{amount} litre yakıt yükleniyor...");
        this.FuelLevel += amount;
    }
}

class Robot
    : Vehicle
{
    public Robot(string name, float fuel)
        : base(name, fuel)
    {
    }

    public void Walk(float x, float y)
    {
        Console.WriteLine($"{x},{y} noktasında hareket halinde");
        this.State = State.OnTheMove;
    }
}

class Submarine
    : Vehicle
{
    public Submarine(string name, float fuel)
        : base(name, fuel)
    {
    }
    public void Dive(int depth)
    {
        Console.WriteLine($"{depth} metreye dalıyor");
        this.State = State.Dive;
    }
}
```

Bu örneği çalıştırdığımızda aşağıdaki gibi sonuçlar alırız.

![../images/oop_3.png](../images/oop_3.png)

Kalıtımı kullanmanın sebepleri arasında tekrarlı kod bloklarını engellemeyi, türler için ortak olan özellik ve fonksiyonellikleri bir noktada toplamayı sayabiliriz. Gerçi ben sınıf bazlı kalıtım yerine Interface kullanmaktan ve ille de gerekiyorsa ortak özellik ve fonksiyonları tutan sınıfların alt sınıflarda birer özellik olarak kullanılmasından yanayım. Ancak bu tartışmaya açık bir konu. Biz şimdi rust cephesinden duruma bir bakalım. Rust tarafında bu tip bir kalıtım formasyonu yok ama ortak fonksiyonellikleri birer davranış gibi düşünürsek trait şeklinde tanımlayabiliriz.

```rust

```