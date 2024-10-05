extern crate core;

use crate::container::Container;
use crate::inventory::Inventory;
use crate::provider::Provider;
use element::Element;

mod container;
mod element;
mod inventory;
mod provider;

fn main() {
    /*
       Generic yapıyı kullandığımızda Inventory veri modelindeki içerik ilk
       kullanılan tipe göre değişir. Örneğin inventory_3 için new fonksiyonuna boolean bir değer
       atandığı için, push metodu da aynı türden bir değer bekler.
    */
    let _element_1 = Element::new(String::from("Mana Crystals"));
    let _element_2 = Element::new(100);
    let mut element_3 = Element::new(false);
    element_3.push(true);

    let mut inventory = Inventory::new(vec![String::from("Mana Gem")]);
    Provider::add_string(&mut inventory, String::from("Gold Metal"));
    Provider::add_string(&mut inventory, String::from("Dark Wood"));
    if let Some(mana) = inventory.get() {
        println!("mana: {}", mana);
    }

    let mut inventory = Inventory::new(vec![1, 6, 9, 10]);
    if inventory.is_empty() {
        println!("No inventory found");
    }

    Provider::add_number(&mut inventory, 10);
    Provider::add_number(&mut inventory, 8);
}
