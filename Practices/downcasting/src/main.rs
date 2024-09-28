use std::any::Any;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/*
    Bu örnekte özellikle farklı türden struct nesnelerini taşıyabilen
    Inventory modeli ve downcasting konusuna bakmaya çalıştım.
*/

fn main() {
    let mut enemy_health = Health(100);
    enemy_health.lose(10);
    println!("Enemy health is {}", enemy_health);

    let player_box = Inventory {
        objects: vec![
            Box::new(Gem { value: 1000 }),
            Box::new(FieldGlass {
                range: 2,
                range_unit: RangeUnit::SeaMile,
            }),
            Box::new(Gem { value: 500 }),
            Box::new(Gem { value: 750 }),
            Box::new(Chest {
                content: String::from("Gemi yapım malzemeleri"),
                origin: String::from("Mars Colony"),
                volume: 3.5,
            }),
        ],
    };

    if let Some(gem) = player_box.get::<Gem>(0) {
        println!("{:?}", gem);
    }

    for object in &player_box.objects {
        if let Some(gem) = object.downcast_ref::<Gem>() {
            println!("{:?}", gem);
        } else if let Some(field_glass) = object.downcast_ref::<FieldGlass>() {
            println!("{:?}", field_glass);
        } else if let Some(chest) = object.downcast_ref::<Chest>() {
            println!("{:?}", chest);
        } else {
            println!("Bilinmeyen bir nesne!");
        }
    }

    let gems: Vec<&Gem> = player_box.get_all::<Gem>();
    for gem in gems {
        println!("Gem: {:?}", gem);
    }

    let field_glasses: Vec<&FieldGlass> = player_box.get_all::<FieldGlass>();
    for field_glass in field_glasses {
        println!("FieldGlass: {:?}", field_glass);
    }
}

struct Health(u32);

impl Health {
    pub fn new(value: u32) -> Self {
        Health(value)
    }

    pub fn is_live(&self) -> bool {
        self.0 > 0
    }

    pub fn lose(&mut self, amount: u32) {
        self.0 -= amount;
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}

impl Deref for Health {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Gem {
    value: u32,
}

#[derive(Debug)]
struct FieldGlass {
    range: u32,
    range_unit: RangeUnit,
}

#[derive(Debug)]
struct Chest {
    origin: String,
    volume: f32,
    content: String,
}

#[derive(Debug)]
enum RangeUnit {
    Km,
    SeaMile,
}

// Herhangibir tipte nesneleri saklamak için bir yol
struct Inventory {
    objects: Vec<Box<dyn Any + 'static>>,
}

impl Inventory {
    pub fn get<T: Any + 'static>(&self, index: usize) -> Option<&T> {
        if index >= self.objects.len() {
            return None;
        }
        self.objects[index].downcast_ref()
    }

    pub fn get_all<T: Any + 'static>(&self) -> Vec<&T> {
        self.objects
            .iter()
            .filter_map(|object| object.downcast_ref::<T>())
            .collect()
    }
}
