/*
   Bu versiyonda ise entity nesneleri container görevi üstlenen bir veri yapısında saklanır.
   Enum kullanımı terk edilmiştir.
*/
use std::time::Instant;

fn main() {
    let game = prepare_objects();
    let start = Instant::now();
    let mut total_cost = 0.;
    for _ in 0..10 {
        total_cost = game.calc_total_cost();
    }
    let duration = start.elapsed() / 10 as u32;
    println!(
        "Total cost is {total_cost}. Total duration is {:?}",
        duration
    );
}

fn prepare_objects() -> Game {
    let mut tanks: Vec<Tank> = Vec::new();
    let mut artilaries: Vec<Artilary> = Vec::new();
    let mut planes: Vec<Plane> = Vec::new();
    for _ in 0..1_000_000 {
        tanks.push(Tank {
            range: 1000.,
            fpk: 4.5,
        });
        tanks.push(Tank {
            range: 750.,
            fpk: 3.6,
        });
        artilaries.push(Artilary { range: 3.6 });
        planes.push(Plane {
            passenger_count: 4,
            range: 780.,
            fpk: 10.50,
        });
    }
    println!(
        "Work withs {} objects",
        tanks.len() + artilaries.len() + planes.len()
    );
    Game {
        tanks,
        artilaries,
        planes,
    }
}

struct Game {
    tanks: Vec<Tank>,
    artilaries: Vec<Artilary>,
    planes: Vec<Plane>,
}

impl Game {
    fn calc_total_cost(&self) -> f32 {
        // let tanks_cost: f32 = self.tanks.iter().map(|t| t.find_range_cost()).sum();
        // let artilaries_cost: f32 = self.artilaries.iter().map(|a| a.find_range_cost()).sum();
        // let planes_cost: f32 = self.planes.iter().map(|p| p.find_range_cost()).sum();
        // tanks_cost + artilaries_cost + planes_cost

        self.tanks
            .iter()
            .map(|t| t.find_range_cost())
            .chain(self.artilaries.iter().map(|a| a.find_range_cost()))
            .chain(self.planes.iter().map(|p| p.find_range_cost()))
            .sum()
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
