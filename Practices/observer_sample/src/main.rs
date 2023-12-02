fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum StockChangedEvent {
    LevelChanged(u32),
}
trait Observer {
    fn on_stock_changed(&self, event: StockChangedEvent);
}

#[derive(Debug, Clone)]
struct Product {
    code: u32,
    stock_level: u32,
    observers: Vec<Box<dyn Observer + Send + Sync>>,
}