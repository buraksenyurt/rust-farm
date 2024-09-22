fn main() {
    let mut mouse = Product::new(1, "Optik Mouse".to_string(), 10.00);
    // Klasik Mutable referans kullanımı (Problem Yok)
    println!("{:?}", mouse);
    discount(&mut mouse, 10.0);
    println!("{:?}", mouse);

    /*
       Problem Senaryosu #00 : T anında sadece tek bir mutable referans olabilir ve hatta
       hiçbir read-only referans da olmamalıdır.

       Aşağıdaki senaryoda mouse referansı immutable olarak mouse_ref'e alınıyor.
       Ardından mouse'un mutable referansı discount fonksiyonuna gönderiliyor.
       Buraya kadar bir sorun olmayacaktı. Ancak, devam eden satırda println fonksiyonunda,
       mouse'un immutable bir referansı kullanılmak istenir. Mouse'un bu t anında bir adet
       mutable referansı ve bir adet immutable referansı söz konusudur. Bu bir ihlaldir ve
       'cannot borrow `mouse` as mutable because it is also borrowed as immutable'
       hatası ile sonuçlanır.
    */

    // let mouse_ref = &mouse; // immutable borrow occurs here
    // discount(&mut mouse, 10.0); //  mutable borrow occurs here
    // println!("{:?}", mouse_ref); //immutable borrow later used here

    /*
       Problem Senaryosu #01: İster immutable, ister mutable olsun ortamda bir değerin referansı
       varsa, orjinal değer sahibinin sahip olduğu değeri değiştirmesine müsade edilmez.

       Aşağıdaki senaryoda println! ile product_ref kullanılmaya çalışılana kadar problem yoktur.

       Ancak 39ncu satır ile 43ncü satır arasında product_ref'in işaret ettiği değer,
       orjinal sahibi tarafından değiştirilemye çalışılmakta. Bu Unexpected Update olarak da ifade
       edilen bir durum. Rust, bu tip beklenmedik güncellemeleri minimize etmeye çalışan kuralları
       içerir.

       Hata mesajını görmek için 48in yorum satırını kaldırın.
    */

    let mut tv = Product::new(
        2,
        "37 Ekran tüplü siyah beyaz televizyon".to_string(),
        45.00,
    );
    let product_ref = &tv; // `tv.title` is borrowed here

    tv.title = "51 Ekran renkli televizyon".to_string(); // tv.title` is assigned to here but it was already borrowed

    // println!("{:?}", product_ref); // borrow later used here

    /*
       Senaryo :

       Ürünlerin listesini tutan bir nesnemiz olsun. Catalog olarak isimlendirelim.
       Kataloğa yeni bir ürün eklemek için de harici bir fonksiyon kullanmak istediğimizi düşünelim.
       Bunu yazmanın birkaç yolu var ancak ideomatic olanı nedir?
    */
    let mut catalog = Catalog::default();
    let bas_guitar = Product::new(3, "Fender Ştratakoster bas gitar".to_string(), 1000.54);
    let org = Product::new(4, "Atarima Org".to_string(), 100.50);
    add_product(&mut catalog, bas_guitar);
    add_product(&mut catalog, org);
    for p in catalog.products {
        println!("{:?}", p);
    }

    /*
        Rust dilinde sayısal değerler, bool, char, elemanları kopylanabilen array'ler,
        elemanları kopyalanabilen tuple'lar, okunabilir veya yazılabilir referanslar
        kopylama suretiyle taşınırlar. Ancak String türü için durum farklıdır. Söz gelimi
        aşağıdaki kod parçasını göz önüne alalım. id ve product_name ayrı birer binding olarak
        hazırlanıyorlar. Sonrasında bu değerlerden yararlanılarak bir Product nesnesi örnekleniyor.
        Ne var ki println! fonksiyonunda 'Value used after being moved [E0382]' hatası alınıyor.
     */
    let id = 6;
    let product_name = String::from("Altılı Kahve Bardağı Seti");
    let _cup = Product::new(id, product_name, 19.90);
    //                  in this method  ^^^^^^ this parameter takes ownership of the value
    // println!("{}->{}", id, product_name); // 'Value used after being moved [E0382]'
}

// Catalog üstünden değişiklik olacağı için mutable referans olarak kullanılabilir
// Product ise bu fonksiyona taşınabilen ve içerideki vektöre eklendikten sonra yok edilebilen
// şekliyle kullanılabilir.
fn add_product(catalog: &mut Catalog, product: Product) {
    catalog.products.push(product);
}

#[derive(Debug, Default)]
struct Catalog {
    products: Vec<Product>,
}

// Mutable Reference kullanımı
fn discount(product: &mut Product, rate: f32) {
    product.list_price -= product.list_price * (rate / 100.)
}

#[derive(Debug)]
struct Product {
    id: u32,
    title: String,
    list_price: f32,
}

impl Product {
    fn new(id: u32, title: String, list_price: f32) -> Self {
        Product {
            id,
            title,
            list_price,
        }
    }
}
