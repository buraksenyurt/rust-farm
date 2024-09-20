fn main() {
    let buji = Product::new(1, "Buji".to_string(), 4500);
    let buji_2 = buji;

    //// Ownership kuralı gereği buji değişkeninin değeri, buji_2'ye taşınmıştır(moved)
    //// Bu nedenle buji'nin işaret ettiği bir değer artık yoktur.
    //// Dolayısıyla değeri olmayan bir değişkeni aşağıdaki gibi bir metoda parametre geçemeyiz
    // discount(buji, 10);

    // Ancak aşağıdaki kullanım geçerlidir. Çünkü buji_2'nin şu anda işaret ettiği bir değer var
    discount(buji_2, 10);

    let mouse = Product::new(2, "Optik Mouse".to_string(), 5500);
    // mouse değeri products isimli vektöre taşınır (moved)
    let products = vec![mouse];
    // Bu nedenle aşağıdaki metot çağrısı için de "value used here after move" hatası alırız
    // discount(mouse, 90);
}

fn discount(mut product: Product, rate: u32) {
    product.list_price -= product.list_price * (rate / 100)
}

#[derive(Debug)]
struct Product {
    id: u32,
    title: String,
    list_price: u32,
}

impl Product {
    fn new(id: u32, title: String, list_price: u32) -> Self {
        Product {
            id,
            title,
            list_price,
        }
    }
}
