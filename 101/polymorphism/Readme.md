# Rust ile Polymorphism Uygulanması

Örneğin ilk halinde aşağıdaki kod söz konusudur.

```rust
fn main() {
    let professor_zundap = FamilyCar;
    //my_car.drive();
    go_trip(&professor_zundap);
    let lightning_mcqueen=Racer;
    go_trip_v2(&lightning_mcqueen);
}

fn go_trip(vehicle: &FamilyCar) {
    vehicle.drive();
}

fn go_trip_v2(vehicle: &Racer) {
    vehicle.drive();
}

struct FamilyCar;

impl FamilyCar {
    fn drive(&self) {
        println!("Sedan yolda");
    }
}

struct Racer;

impl Racer{
    fn drive(&self) {
        println!("Yarışçı yolda");
    }
}
```

Bu örnekte drive işlevine sahip iki veri modeli söz konusudur. go_trip fonksiyonu her iki nesne modeli için tekrar yazılmıştır. C# , Java gibi dillerde bu fonksiyona drive işlevini taşıyan bir interface aktarılarak çok biçimli bir çalışma kabiliyeti kazanması sağlanabilir. Rust tarafında bunun için trait'lerden yararlanılır. İki seçenek vardır. Birisi dynamic dispatch diğeri ise static dispatch kullanımıdır. Kodun buna göre düzenlenmiş hali aşağıdaki gibidir.

```rust
trait VehicleCapability {
    fn drive(&self);
}
fn main() {
    let professor_zundap = FamilyCar;
    let lightning_mcqueen = Racer;
    
    go_trip(&professor_zundap);
    go_trip(&lightning_mcqueen);

    // static dispatch kullanımı
    go_trip_v2(&professor_zundap);
    go_trip_v2(&lightning_mcqueen);
}

// dynamic dispatch
// çalışma zamanında hangi türün kullanıldığı belirlenir
// ve çağrılacak metot dinamik olarak seçilir.
// Bu tekniğin performans açısından bir maliyeti vardır.
// Çalışma zamanında tür belirleme ve dinamik ömür gereken durumlar için elverişlidir.
fn go_trip(vehicle: &dyn VehicleCapability) {
    vehicle.drive();
}

// static dispatch
// Bu teknik performans açısından daha iyidir.
// Derleme zamanında tür belirleme ve performans optimizasyonu gerektiğinde tercih edilir.
fn go_trip_v2(vehicle: &impl VehicleCapability) {
    vehicle.drive();
}

struct FamilyCar;
struct Racer;

impl VehicleCapability for FamilyCar {
    fn drive(&self) {
        println!("Sedan yolda");
    }
}
impl VehicleCapability for Racer {
    fn drive(&self) {
        println!("Yarışçı yolda");
    }
}
```

Örneğin son bölümünde ise birden fazla davranışını barındırabilen super trait örneği incelenmiştir.

```rust
// İstersek trait içerisinde varsayılan bir davranış da uyarlayabiliriz.
// Bu durumda kendi veri modelimizde ezmezsek varsayılan versiyon çalışır
trait VehicleCapability2 {
    fn drive(&self) {
        println!("Araç yolda");
    }
}

trait WeaponCapability {
    fn fire(&self) {
        println!("Ateş ediyor");
    }
}

// Military bir super trait'tir.
// WeaponeCapability ile VehicleCapability2 trait'lerini de taşır
trait Military: WeaponCapability + VehicleCapability2 {}
struct Tank;
// Tank veri modeli Military isimli super trait'i uyguladığı için
// hem WeaponCapability hem de VehicleCapability2 trait'lerini
// implemente etmek zorundadır.
impl WeaponCapability for Tank {}
impl VehicleCapability2 for Tank {}
impl Military for Tank {}

// Hem VehicleCapability hem WeaponCapability davranışlarını barındıran
// Military davranışını uygulayan her veri modeli bu fonksiyona gelebilir.
fn go_to_battle(vehicle: &impl Military) {
    vehicle.drive();
    vehicle.fire();
}

fn main() {
    // super trait kullanım örneği
    let soldir_of_fortune = Tank;
    go_to_battle(&soldir_of_fortune);
}
```

