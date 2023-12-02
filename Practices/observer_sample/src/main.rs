/*
   Elimde Product isimli bir veri modeli var. İçinde stok seviyesini tutan bir de alan var.
   Bu alan değiştiğinde(bir başka deyişle stok seviyesi değiştiğinde) bir event oluşturmak
   ve bu event'i dinleyen kısımda da değişikliği algılamak istiyorum.

   DotNet tarafında event ve delegate tiplerini bunun için kullanabiliyoruz.
*/
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

fn main() {
    let mut product = Product::new(32, 10, 120.10);
    let observer = Box::new(ProductObserver);

    product.subscribe(observer);
    product.update_stock_level(12);
    product.update_stock_level(6);
    product.apply_discount(10.10);
}

struct ProductObserver;
impl Observer for ProductObserver {
    fn on_stock_changed(&self, event: Event) {
        if let Event::StockLevelChanged(old_level, new_level) = event {
            println!(
                "Stok seviyesi {} birimden {} birime değiştirildi",
                old_level, new_level
            );
        }
    }

    fn on_apply_discount(&self, event: Event) {
        if let Event::AppliedDiscount(discount_value) = event {
            println!("{} birim indirim uygulandı", discount_value);
        }
    }
}

// Değişikliği tarifleyen bir enum
#[derive(Debug)]
enum Event {
    StockLevelChanged(u32, u32),
    AppliedDiscount(f32),
}

// Event' leri tarifleyen Observer trait'i
trait Observer {
    fn on_stock_changed(&self, event: Event);
    fn on_apply_discount(&self, event: Event);
}

// Kobay veri modelimiz
struct Product {
    code: u32,
    stock_level: Arc<Mutex<u32>>,
    unit_price: Arc<Mutex<f32>>,
    observers: Vec<Box<dyn Observer + Send + Sync>>,
}

impl Product {
    fn new(code: u32, stock_level: u32, unit_price: f32) -> Self {
        Self {
            code,
            stock_level: Arc::new(Mutex::new(stock_level)),
            unit_price: Arc::new(Mutex::new(unit_price)),
            observers: Vec::new(),
        }
    }

    // Aslında Observer implementasyonunu eklediğimiz yer
    fn subscribe(&mut self, observer: Box<dyn Observer + Send + Sync>) {
        self.observers.push(observer);
    }

    fn update_stock_level(&self, new_level: u32) {
        let mut level = self.stock_level.lock().unwrap();
        let old_level = *level;
        *level = new_level;

        for observer in &self.observers {
            observer.on_stock_changed(Event::StockLevelChanged(old_level, new_level));
        }
    }

    fn apply_discount(&self, value: f32) {
        let mut unit_price = self.unit_price.lock().unwrap();
        *unit_price -= value;
        for observer in &self.observers {
            observer.on_apply_discount(Event::AppliedDiscount(value));
        }
    }
}
