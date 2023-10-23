/*
   Bu versiyonda trait ve Box yerine enum kullanılıyor.
   Plane, Artilary ve Tank için maliyet hesabı yapan fonksiyonlar.
   Entity ise bir enum sabiti. Buna uygulanan calc_range_cost hangi türü barındırıyorsa
   onun find_range_cost fonksiyonunu çağırmakta.
   Vector listemiz Entity türünden elamanlar barındıryor. Listeyi dolaşan calc_total_cost
   fonksiyonu her bir entity için calc_range_cost fonksiyonunu çağırmakta. Bu da match ifadesine
   göre doğru olan find_range_cost fonksiyonunu çağırıyor.

   Bu örnekte Box ile heap üstüne işlem yapılmadığından toplam çalışma süresi 00 versiyonuna göre
   elbette daha kısa.
*/
use std::time::Instant;

fn main() {
    let entity_list = prepare_objects();
    let start = Instant::now();
    let mut total_cost = 0.;
    for _ in 0..10 {
        total_cost = calc_total_cost(&entity_list);
    }
    let duration = start.elapsed() / 10 as u32;
    println!(
        "Total cost is {total_cost}. Total duration is {:?}",
        duration
    );
}

fn calc_total_cost(objects: &[Entity]) -> f32 {
    objects
        .iter()
        .fold(0., |total, entity| total + entity.calc_range_cost())
}

fn prepare_objects() -> Vec<Entity> {
    let mut objects: Vec<Entity> = Vec::with_capacity(1_000_000);
    for _ in 0..1_000_000 {
        objects.push(Entity::Tank(Tank {
            range: 1000.,
            fpk: 4.5,
        }));
        objects.push(Entity::Tank(Tank {
            range: 750.,
            fpk: 3.6,
        }));
        objects.push(Entity::Artilary(Artilary { range: 3.6 }));
        objects.push(Entity::Plane(Plane {
            passenger_count: 4,
            range: 780.,
            fpk: 10.50,
        }));
    }
    println!("Works withs {} objects.", objects.len());
    objects
}

enum Entity {
    Tank(Tank),
    Artilary(Artilary),
    Plane(Plane),
}

impl Entity {
    fn calc_range_cost(&self) -> f32 {
        match self {
            Entity::Tank(tank) => tank.find_range_cost(),
            Entity::Artilary(artilary) => artilary.find_range_cost(),
            Entity::Plane(plane) => plane.find_range_cost(),
        }
    }
}

struct Tank {
    range: f32,
    fpk: f32, // fuel per km
}

impl Tank {
    fn find_range_cost(&self) -> f32 {
        self.range * self.fpk
    }
}

struct Artilary {
    range: f32,
}

impl Artilary {
    fn find_range_cost(&self) -> f32 {
        self.range
    }
}

struct Plane {
    range: f32,
    passenger_count: u16,
    fpk: f32,
}

impl Plane {
    fn find_range_cost(&self) -> f32 {
        self.range * self.fpk + (self.passenger_count as f32 * 1.5)
    }
}
