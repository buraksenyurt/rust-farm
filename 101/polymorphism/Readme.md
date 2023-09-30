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

Bu örnekte drive işlevine sahip iki veri modeli söz konusudur. go_trip fonksiyonu her iki nesne modeli için tekrar yazılmıştır. C# , Java gibi dillerde bu fonksiyona drive işlevini taşıyan bir interface aktarılarak çok biçimli bir çalışma kabiliyeti kazanması sağlanabilir. Rust tarafında bunun için trait'lerden yararlanılır.

