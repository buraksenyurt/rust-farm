/*
   #00 - Başlangıç Senaryosu

   create_product, gelen parametreleri kullanarak bir Product nesnesi örnekler ve
   oluşturula değeri işaret eden referansı döndürür. Ancak bu Lifetime hatasına sebep olur.

*/

// fn main() {
//     let sun_glass = create_product("Güneş gölüğü".to_string(), 100.00);
//     println!("{:?}", sun_glass);
// }
//
// fn create_product(title: String, list_price: f32) -> &Product {
//     let product = Product {
//         id: 1,
//         title,
//         list_price,
//     };
//     &product
// }

#[derive(Debug)]
struct Product {
    id: u32,
    title: String,
    list_price: f32,
}

impl Product {
    fn new(id: u32, title: String, list_price: f32) -> Self {
        Self {
            id,
            title,
            list_price,
        }
    }

    fn discount(&mut self, amount: f32) -> f32 {
        self.list_price -= amount;
        self.list_price
    }

    fn get_summary(&self) -> String {
        format!("{}. {}-{} coin", self.id, self.title, self.list_price)
    }
}

#[derive(Debug)]
struct Catalog {
    products: Vec<Product>,
}

impl Catalog {
    fn new() -> Self {
        Self { products: vec![] }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn total_value(&self) -> f32 {
        self.products.iter().map(|p| p.list_price).sum()
    }

    fn get_summaries(&self) -> Vec<String> {
        self.products
            .iter()
            .map(|p| p.get_summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    /*
        #01 Aşağıdaki durumu inceleyelim.

        Catalog nesnesi örnekleniyor. Sonrasında bir Product oluşturuyoruz.
        Bu Product nesnesini add_product metodu ile kataloğa ekliyoruz.
        Ancak burada need_for_speed değişkeni add_product metoduna taşınıyor (moved)
        Dolayısıyla add_product metodu kapsamını tamamladığında yok ediliyor(dropped)
        Bu durumda need_for_speed üstünden discount metodunu çağırmak istediğimizde
        'Value used after being moved [E0382]' hatası alıyoruz.

    */
    let mut game_catalog = Catalog::new();
    let mut need_for_speed = Product::new(1, "Need for Speed 98".to_string(), 9.99);
    game_catalog.add_product(need_for_speed);
    // let _discounted_price = need_for_speed.discount(1.0);

    println!("{:?}", game_catalog);
}
