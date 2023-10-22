/*
    Örnekte Tank, Artilary, Plane isimli 3 veri yapısı var.
    Her birinin bir görev sırasındaki maliyetlerini çıkarmak için bir fonksiyon kullanılıyor.
    Bu fonksiyon bir trait olarak tanımlanmış durumda. Bu nesneler için de ortak bir davranış
    olarak tanımlanmış durumda. Dolayısıyla bu davranışı taşıyanların listesinin olduğu bir
    vektörde toplam maliyeti hesaplayacak bir fonksiyon yazmak(calc_total_cost) kolay.
    İlk versiyonda bu işlemin toplam süresine bir bakılıyor.
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

fn calc_total_cost(objects: &[Box<dyn Entity>]) -> f32 {
    objects
        .iter()
        .fold(0., |total, entity| total + entity.find_range_cost())
}

fn prepare_objects() -> Vec<Box<dyn Entity>> {
    let mut objects: Vec<Box<dyn Entity>> = Vec::with_capacity(1_000_000);
    for _ in 0..1_000_000 {
        objects.push(Box::new(Tank {
            range: 1000.,
            fpk: 4.5,
        }));
        objects.push(Box::new(Tank {
            range: 750.,
            fpk: 3.6,
        }));
        objects.push(Box::new(Artilary { range: 3.6 }));
        objects.push(Box::new(Plane {
            passenger_count: 4,
            range: 780.,
            fpk: 10.50,
        }))
    }
    objects
}

trait Entity {
    fn find_range_cost(&self) -> f32;
}

struct Tank {
    range: f32,
    fpk: f32, // fuel per km
}

impl Entity for Tank {
    fn find_range_cost(&self) -> f32 {
        self.range * self.fpk
    }
}

struct Artilary {
    range: f32,
}

impl Entity for Artilary {
    fn find_range_cost(&self) -> f32 {
        self.range
    }
}

struct Plane {
    range: f32,
    passenger_count: u16,
    fpk: f32,
}

impl Entity for Plane {
    fn find_range_cost(&self) -> f32 {
        self.range * self.fpk + (self.passenger_count as f32 * 1.5)
    }
}
