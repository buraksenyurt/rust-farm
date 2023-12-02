use std::sync::{Arc, Mutex};
/*
   Örnekteki amaç bir vektör üzerinde olan değişiklikleri gözlemleyebilmek.
*/

fn main() {
    let mut products: Database<Product> = Database::new();

    /*
        products değişkeni ile temsil edilen Database veri yapısındaki
        data vecktörüne yeni eleman eklenince subscribe ettiğimiz fonksiyon çalıştırılacaktır.
    */
    products.subscribe(|event| match event {
        DatabaseEvent::Add(p) => {
            println!(
                "{} kodlu yeni bir ürün eklendi. Stok seviyesi {}.",
                p.code, p.stock_level
            );
        }
        _ => {}
    });

    products.add(Product {
        code: 1,
        stock_level: 10,
    });
    products.add(Product {
        code: 2,
        stock_level: 8,
    });
    products.add(Product {
        code: 3,
        stock_level: 22,
    });
}

enum DatabaseEvent<T> {
    Add(T),
    Delete(T),
    Update(T),
}

struct Database<T> {
    // Asıl datayı tuttuğumuz vektör. Burayı gözlemleyebilmek istiyoruz.
    data: Arc<Mutex<Vec<T>>>,

    // Oberserver'ları aşağıdaki vektörde toplayacağız
    // dynamic dispatch söz konusu. çalışma zamanında hangi işlevin çağırılacağı belirlenecek
    // Fn trait'i fonksiyon gibi davranabilen trait'leri temsil eder
    // Send bir tipin başka bir thread'e gönderilebilceğini belirtir
    // Sync ise bir tipin birden fazla thread tarafından aynı anda kullanılabileceğini belirtir
    // Yani Fn() ile bir işlevi çağırabilir ve farklı thread'lerde kullanabilir hale geliyoruz.
    observers: Vec<Box<dyn Fn(DatabaseEvent<T>) + Send + Sync>>,
}

impl<T> Database<T>
where
    T: Clone,
    T: Copy,
{
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
            observers: Vec::new(),
        }
    }
    // Yeni bir öğenin güvenli bir şekilde eklenmesi sağlanır
    fn add(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        data.push(item);
        // ayrıca var olan observer fonksiyonları da tetiklenir
        // observer'ları subscribe metodu ile observers listesine ekleriz
        // Bu birden fazla yerden observer fonksiyonları tanımlayıp
        // değişiklikleri izleyebileceğimiz anlamına gelir
        for observer in &self.observers {
            observer(DatabaseEvent::Add(item.clone()));
        }
    }

    fn get_data(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data.lock().unwrap().clone()
    }

    fn subscribe<F>(&mut self, observer: F)
    where
        F: Fn(DatabaseEvent<T>) + Send + Sync + 'static,
    {
        self.observers.push(Box::new(move |event| {
            observer(event);
        }));
    }
}

#[derive(Debug, Clone, Copy)]
struct Product {
    code: u32,
    stock_level: u32,
}
