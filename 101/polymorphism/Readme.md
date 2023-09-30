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

