/*
   Enum türü ile çalışmayı hatırlamak.

   Elektronik, market ve giyim ile ilgili veri yapıları tasarlamak istediğimiz düşünelim.
   OOP tarafında çalışanlar için Rust dünyasında ilk akla gelen seçenek Struct olacaktır.
   Ancak bazı hallerde enum türü işe yarar ve varyantlar bir enum altında birleştirilebilir.

   Genel sorulardan birisi ne zaman enum ne zaman struct kullanılacağına karar vermektir.
   Aynı metotlara sahip veri yapıları söz konusu olduğuna enum tercih edilebilir.
   Farklılaşan metotlar varsa struct daha iyi bir tercih olacaktır.
*/
fn main() {
    let raspy = Product::Electronics {
        name: "Raspberry 8Gb".to_string(),
        brand: "Raspberry".to_string(),
        warranty_years: 10,
    };
    // println!("{:#?}", raspy);
    // println!("{}", raspy.summary());
    println!("{}", raspy.summary2());

    let c64_shirt = Product::Clothing {
        name: "Commodore 64 T-Shirt".to_string(),
        color: "Black".to_string(),
        size: "L".to_string(),
    };
    // println!("{:#?}", c64_shirt);
    //println!("{}", c64_shirt.summary());
    println!("{}", c64_shirt.summary2());

    let nugat = Product::Grocery {
        name: "Nugat".to_string(),
        expiry_date: "01.01.2004".to_string(),
    };
    // println!("{}", nugat.summary());
    println!("{}", nugat.summary2());

    // let products = vec![raspy, c64_shirt, nugat];
    // for p in products.iter() {
    //     // println!("{:?}", p);
    //     println!("{}", p.summary());
    // }

    let mut your_market = Shop::new();
    your_market.add(nugat);
    your_market.add(c64_shirt);
    your_market.add(raspy);
    your_market.add(Product::DiscountCoupon(40.50));
    println!("{:#?}", your_market);

    /*
       Rust bilindiğin üzere null, nil veya undefined konseptlerine sahip değildir.
       Bu noktada Option enum türünün önem kazandığını söyleyebiliriz.
       Option sadece iki varyanta sahiptir.

       enum Option {
           Some(value),
           None
       }

       Option kullanımı bizi tüm olasılıkları kontrol etmeye de zorlar.
    */
    let index = 1000;
    println!("{:?}", your_market.products.get(index)); // Bu eleman olmadığından None varyantı oluşur
    match your_market.products.get(index) {
        None => {
            println!("There is no product in this index {}", index)
        }
        Some(value) => {
            println!("{}", value.summary2());
        }
    }

    let index = 2;
    match your_market.get2(index) {
        Some(p) => {
            println!("{}", p.summary())
        }
        None => {
            println!("There is no product in this index {}", index)
        }
    }
    /*
       Option enum'ını kullandığımızda yukarıdaki gibi pattern matching ifadeleri ile
       çıktıyı kontrol altına alabiliriz. Bunun yanında ekstra yollar da vardır.

       - unwrap() :
                   En kısa yollardan birisi. Doğrudan değeri elde ederiz. Ama ortada bir değer
                   yoksa, panic üretiliri.
                   Genellikle değerleri debug ederken kolayca anlamak için kullanılabilir
       - expect("error message") :
                   unwrap gibi değer olmadığında panic üretir ancak parametre olarak verilen
                   bilgiyi de basar. Bir değer olmadığında programın çökmesini (crash) istersek
                   kullanabiliriz.
       - unwrap_or(&variable)  :
                   Eğer değer yoksa varsayılan bir değer dönülmesini sağlar.
                   Eğer None karşılığında kullanılabilecek yedek bir değer (Fallback value)
                   söz konusu ile ele alınabilir
    */

    // // 9000nic indekste elemanımız olmadığından ve unwrap kullanıldığından panic oluşur
    // // Debug amaçlı kullanalım
    // let index = 9000;
    // let product = your_market.get2(index).unwrap();
    // println!("{}", product.summary2());

    // // 9000nic indekste elemanımız olmadığından ve expect kullanıldığından panic oluşur
    // // ve bizim ilettiğimiz mesaj yazdırılır.
    // // Özellikle üretim ortamlarından programın kesinlikle crash edilmesini istersek kullanalım.
    // let index = 9000;
    // let product = your_market.get2(index).expect("Index not found!!!");
    // println!("{}", product.summary2());

    // 9000nic indekste elemanımız olmadığından ve unwrap_or kullanıldığından
    // bizim söylediğimiz ve None yerine geçecek default değer kullanılır.
    let index = 9000;
    let out_of_stock = Product::OutOfService;
    let product = your_market.get2(index).unwrap_or(&out_of_stock);
    println!("{}", product.summary2());

    let index = 0;
    let out_of_service = Product::OutOfService;
    let product = your_market.get2(index).unwrap_or(&out_of_service);
    println!("{}", product.summary2());

    // let index = 2;
    // match your_market.get(index) {
    //     ProductResult::ThereIs(p) => {
    //         println!("{}", p.summary())
    //     }
    //     ProductResult::None => {
    //         println!("There is no product in this index {}", index)
    //     }
    // }
}

// 00 altında belirtilen veri modelindeki her şey bir ürün ama özellikleri farklı.
// Bunları Product isimli bir enum sabitinde aşağıdaki gibi tasarlayabiliriz de.
#[derive(Debug)]
enum Product {
    Electronics {
        // Variant
        name: String,
        brand: String,
        warranty_years: u8,
    },
    Grocery {
        // Variant
        name: String,
        expiry_date: String,
    },
    Clothing {
        // Variant
        name: String,
        size: String,
        color: String,
    },
    DiscountCoupon(f32), // Unlabeled Field
    OutOfService,
}

/*
   Enum veri yapısının güzel yanlarından birisi, enun içinde yer alan her varyant için
   ortak fonksiyonellik sağlanabilmesidir. Aşağıdaki fonksiyon bunun bir örneğidir.
*/
impl Product {
    /*
        Enum içerisindeki veri yapıları summary metodundaki gibi
        if let blokları ile kontrol edilebilir.

        Ancak daha şık olan çözüm Pattern Matching kullanmak olacaktır.
        (summary2 metoduna bakın)
    */

    fn summary(&self) -> String {
        if let Product::Electronics {
            name,
            brand,
            warranty_years,
        } = self
        {
            format!(
                "{} ({}). Warranty duration is {} years",
                name, brand, warranty_years
            )
        } else if let Product::Grocery { name, expiry_date } = self {
            format!("{}. Last usage date is '{}'", name, expiry_date)
        } else if let Product::Clothing { name, size, color } = self {
            format!("{} ({}) color is {}", name, size, color)
        } else if let Product::DiscountCoupon(amount) = self {
            format!("You have {} coin discount", amount)
        } else {
            String::from("Unknown")
        }
    }

    fn summary2(&self) -> String {
        match self {
            Product::Electronics {
                name,
                brand,
                warranty_years,
            } => {
                format!(
                    "{} ({}). Warranty duration is {} years",
                    name, brand, warranty_years
                )
            }
            Product::Grocery { name, expiry_date } => {
                format!("{}. Last usage date is '{}'", name, expiry_date)
            }
            Product::Clothing { name, size, color } => {
                format!("{} ({}) color is {}", name, size, color)
            }
            Product::DiscountCoupon(amount) => {
                format!("You have {} coin discount", amount)
            }
            Product::OutOfService => "Vacation time. The shop is closed!".to_string(),
        }
    }
}

#[derive(Debug)]
struct Shop {
    products: Vec<Product>,
}

impl Shop {
    fn new() -> Self {
        Self { products: vec![] }
    }

    fn add(&mut self, product: Product) {
        self.products.push(product);
    }

    /*
       Option yerine kendi enum türlerimiz ile de fonksiyon dönüşlerini kontrol altına alabiliriz.
    */
    fn get(&self, index: usize) -> ProductResult {
        if self.products.len() > index {
            ProductResult::ThereIs(&self.products[index])
        } else {
            ProductResult::None
        }
    }
    // Rust Option enum türünü sağladığından ProductResult gibi bir enum yazmamıza gerek yoktur
    fn get2(&self, index: usize) -> Option<&Product> {
        if self.products.len() > index {
            Some(&self.products[index])
        } else {
            None
        }
    }
}

enum ProductResult<'a> {
    // ThereIs varyantından referans döndüğümüz için lifetime kullanmak durumundayız
    ThereIs(&'a Product),
    None,
}

/*
   Enum türü kullanımı örnekleri farklılaştırabilir.
   Örneğin spor müsabakaları, bir oyundaki ya da eğitimdeki başarımlar, araç vs
   Aşağıda birkaç farklı örnek de yer alıyor.
*/

enum SportEvent {
    Basketball {
        team1: String,
        team2: String,
        points: (u16, u16),
    },
    Volleyball {
        home_team: String,
        away_team: String,
        sets_won: (u8, u8),
    },
    TableTennis {
        player1: String,
        player2: String,
        sets_won: (u8, u8),
    },
    Today {
        title: String,
        what_happened: String,
        year: u8,
        month: u8,
        day: u8,
    },
}

enum Vehicle {
    Airplane {
        airline: String,
        flight_number: String,
        max_capacity: u8,
        max_altitude: u8,
    },
    Car {
        make: String,
        model: String,
        horse_power: u8,
        year: u16,
    },
    Bicycle {
        brand: String,
        gear_count: u8,
        color: String,
    },
    Boat {
        brand: String,
        range: u8,
        builder: String,
        capacity: u8,
    },
}

// 00 struct kullanarak model tasarımı
struct Electronic {
    name: String,
    brand: String,
    warranty_years: u8,
}
struct Grocery {
    name: String,
    expiry_date: String,
}
struct Clothing {
    name: String,
    size: String,
    color: String,
}
