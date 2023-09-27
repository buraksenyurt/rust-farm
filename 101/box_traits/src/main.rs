// use crate::AgentList::{Cons, Nil};

use crate::Agent::{Cons, Nil};

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
    // let agents=Cons(1,Cons(2,Cons(3,Nil)));

    // Box kullanarak veri boyutunu açıkça belirtebiliriz
    let agents = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum Agent {
    Cons(i32, Box<Agent>),
    Nil,
}
enum AgentList {
    Cons(i32, AgentList),
    Nil,
}

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
