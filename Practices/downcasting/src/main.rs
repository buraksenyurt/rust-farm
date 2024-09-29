mod component;

use component::prelude::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
/*
    Bu örnekte özellikle farklı türden struct nesnelerini taşıyabilen
    Inventory modeli ve downcasting konusuna bakmaya çalıştım.
    Ayrıca Any, TypeId türleri ile temel bir ECS(Entity Component System) için gerekli olan
    component setine sahip entity'ler için bir girizgah da yer almakta.
*/

fn main() {
    let mut enemy_health = Health::default();
    enemy_health.lose(10);
    println!("Enemy health is {}", enemy_health);

    let player_box = Inventory {
        objects: vec![
            Box::new(Gem { value: 1000 }),
            Box::new(FieldGlass {
                range: 5,
                range_unit: RangeUnit::SeaMile,
            }),
            Box::new(FieldGlass {
                range: 1,
                range_unit: RangeUnit::Km,
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

    /*
       Bir ECS sisteminde entity'ler kendilerine atanmış componentler ve benzersiz bir id ile
       ifade edilebilirler. Aşağıdaki Any type kullanarak bunu nasıl yapabileceğimizin bir örneği
       var.
    */
    let mut components: HashMap<TypeId, Vec<Box<dyn Any + 'static>>> = HashMap::new();
    let entity_id = TypeId::of::<u32>();
    components.insert(
        entity_id,
        vec![
            Box::new(Health::new(50)),
            Box::new(Position::new(10.0, 10.0)),
            Box::new(Velocity::new(1.0, 0.0)),
        ],
    );
    components.insert(
        TypeId::of::<i32>(),
        vec![
            Box::new(Health::default()),
            Box::new(Position::new(10.0, 10.0)),
            Box::new(Power::new(6)),
        ],
    );
    for (id, comps) in &components {
        println!("{:?}", id);
        for comp in comps {
            println!("{:?}", comp.downcast_ref::<Position>());
        }
    }
}
