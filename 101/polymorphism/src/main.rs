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
