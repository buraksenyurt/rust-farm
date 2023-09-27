// use crate::AgentList::{Friends, Nil};

use crate::AgentRc::{Friends, Nil};
use std::rc::Rc;
//use crate::Agent::{Friends, Nil};

fn main() {
    let player: Box<dyn GameObject> = Box::new(Player {
        name: String::from("My Hero"),
        health: 100,
    });

    let enemy: Box<dyn GameObject> = Box::new(Enemy {
        kind: String::from("Ork"),
        strength: 45,
    });

    let game_objects: Vec<Box<dyn GameObject>> = vec![player, enemy];

    for obj in game_objects {
        obj.draw();
    }

    // Normalde stack'e alınan bir nesneyi heap üstünde konumlandırabiliriz de
    let mut inv: Box<Invoice> = Box::new(Invoice::new(
        101,
        "Ekran Kartı faturası".to_string(),
        1,
        1950.50,
        "1.1.2023".to_string(),
    ));
    inv.set_discount(1.5);
    println!("{:?}", *inv);

    // // Aşağıdaki kullanımda derleme zamanında.
    // // error[E0072]: recursive type `AgentList` has infinite size
    // // hatası alınır.
    // // Önerilerde ise
    // //  insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    // // yazar.
    // let agents=Friends(1,Friends(2,Friends(3,Nil)));

    // Box kullanarak veri boyutunu açıkça belirtebiliriz
    // let agents = Friends(1, Box::new(Friends(2, Box::new(Friends(3, Box::new(Nil))))));

    // // Yukarıda tek sahipliğin olduğu bir durum söz konusu.
    // // Senaryoyu aşağıdaki gibi değiştirelim.
    // // blue_agents ile green_barets red_agents'ı ortaklaşa kullanmak istiyorlar
    // // ne var ki blue_agents oluşturulurken red_agents sahipliği de taşınıyor
    // // ve dolayısıyla sonraki satırda kullanılamaz hale geliyor.
    // let red_agents = Friends(1, Box::new(Friends(2, Box::new(Friends(3, Box::new(Nil))))));
    // let blue_agents = Friends(4, Box::new(red_agents)); //value moved here
    // let green_barets = Friends(5, Box::new(red_agents)); //value used here after move

    // Bu tip bir senaryo için çözüm Rc(Reference Counted) Smart Pointer kullanılabilir.
    let red_agents = Rc::new(Friends(1, Rc::new(Friends(2, Rc::new(Friends(3, Rc::new(Nil)))))));
    let blue_agents = Friends(4, Rc::clone(&red_agents)); //value moved here
    println!("{:?}", blue_agents);
    let green_barets = Friends(5, Rc::clone(&red_agents)); //value used here after move
    println!("{:?}", green_barets);
}

enum Agent {
    Friends(i32, Box<Agent>),
    Nil,
}
#[derive(Debug)]
enum AgentRc {
    Friends(i32, Rc<AgentRc>),
    Nil,
}
// enum AgentList {
//     Friends(i32, AgentList),
//     Nil,
// }

#[derive(Debug)]
struct Invoice {
    id: i32,
    title: String,
    amount: i32,
    price: f32,
    date: String,
}

impl Invoice {
    fn new(id: i32, title: String, amount: i32, price: f32, date: String) -> Self {
        Self {
            id,
            title,
            amount,
            price,
            date,
        }
    }
    fn set_discount(&mut self, discount_amount: f32) {
        self.price -= discount_amount;
    }
}

trait GameObject {
    fn draw(&self);
}

struct Player {
    name: String,
    health: i32,
}
struct Enemy {
    kind: String,
    strength: i32,
}

impl GameObject for Player {
    fn draw(&self) {
        println!("Oyuncu: {} (Sağlık: {})", self.name, self.health);
    }
}

impl GameObject for Enemy {
    fn draw(&self) {
        println!("Düşman: {} (Güç: {})", self.kind, self.strength);
    }
}
