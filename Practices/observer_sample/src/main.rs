/*
   Elimde Product isimli bir veri modeli var. İçinde stok seviyesini tutan bir de alan var.
   Bu alan değiştiğinde(bir başka deyişle stok seviyesi değiştiğinde) bir event oluşturmak
   ve bu event'i dinleyen kısımda da değişikliği algılamak istiyorum.

   DotNet tarafında event ve delegate tiplerini bunun için kullanabiliyoruz.
*/
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

fn main() {
    let mut product = Product::new(32, 10);
    let logger = Box::new(StockChangeLogger);

    product.observe_stock_change(logger);
    product.update_stock_level(12);
    product.update_stock_level(6);
}

struct StockChangeLogger;
impl Observer for StockChangeLogger {
    fn on_stock_changed(&self, event: StockChangedEvent) {
        match event {
            StockChangedEvent::LevelChanged(old_level, new_level) => {
                println!(
                    "Stok seviyesi {} birimden {} birime değiştirildi",
                    old_level, new_level
                );
            }
        }
    }
}

// Değişikliği tarifleyen bir enum
#[derive(Debug)]
enum StockChangedEvent {
    LevelChanged(u32, u32),
}

trait Observer {
    fn on_stock_changed(&self, event: StockChangedEvent);
}

struct Product {
    code: u32,
    stock_level: Arc<Mutex<u32>>,
    observers: Vec<Box<dyn Observer + Send + Sync>>,
}

impl Product {
    fn new(code: u32, stock_level: u32) -> Self {
        Self {
            code,
            stock_level: Arc::new(Mutex::new(stock_level)),
            observers: Vec::new(),
        }
    }

    fn observe_stock_change(&mut self, observer: Box<dyn Observer + Send + Sync>) {
        self.observers.push(observer);
    }

    fn update_stock_level(&self, new_level: u32) {
        let mut level = self.stock_level.lock().unwrap();
        let old_level = *level;
        *level = new_level;

        for observer in &self.observers {
            observer.on_stock_changed(StockChangedEvent::LevelChanged(old_level, new_level));
        }
    }
}
