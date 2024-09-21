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

    // Bir değere istediğimiz kadar immutable referans bağlayabiliriz.
    // Aynen aşağıdaki gibi.
    let tv = Product::new(45, "37 Ekran siyah beyaz".to_string(), 100056);

    let tv_ref_1 = &tv; // burada tv referansı tv_ref_1 tarafından alınıyor
    let tv_ref_2 = &tv;

    // let _lcd = tv; // Burada ise tv değeri tv_ref_1 de iken değer taşıması (move) yapılmaya çalışıyor.
    // // Doğal olarak aşağıdaki çağrımlarda referans değerlerine ulaşılamaması söz konusu olabilir.
    // // tv_ref_1'in referans ettiği değerin boşaldığını düşünelim. Boşalmış bir değeri aşağıdaki
    // // gibi başka bir yerde kullanılmak üzere taşımaya Rust izin vermeyecektir.
    // print_product(tv_ref_1); // borrow later used here
    // print_product(tv_ref_2);
}

fn print_product(product: &Product) {
    println!("{:?}", product);
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
