fn main() {
    //#1 Raw Identifier Örneği. Keyword'lerin field isimlerinde kullanılabilmesi.
    let x = Value::new(10, 10);
    println!("{:?}", x);
    println!("{:#?}", x);
    println!("{} {}", x.r#in, x.r#for);

    //#2 i32 scalar tipi için atamalar ve bellek adresleri
    let number1 = 32;
    let number2 = &number1;
    let number3 = &number2;
    let number5 = number1;

    println!("number1 value {} and address {:p}", number1, &number1);
    println!("number2 value {} and address {:p}", number2, number2);
    println!("number3 value {} and address {:p}", number3, number3);
    println!("number5 value {} and address {:p}", number5, &number5);

    let number4 = &&23;
    println!("number4 address {:p}", &&number4);
    println!("number4 address {:p}", number4);
    println!("number4 value {}", number4);

    println!("x address {:p}", &x);

    // #3 İlişkili fonksiyon ve metot farkı
    // self kullanılmayan struct fonksiyonları Associated Function olarak anılır ve
    // aşağıdaki gibi çağırılırlar.
    Database::get_stats();

    let mut sql = Database::new();
    println!("{:#?}", sql);
    // Method kavramı ise nesne örneği üzerine erişilen işlev olarak düşünülebilir
    sql.setup("lizbon".to_string(), 1234);
    println!("{:#?}", sql);
}

// Raw Identifier Örneği

// Aşağıdaki kod parçası derlenmez nitekim in ve for özel kelimelerdir
// struct Value{
//     for:i32,
//     in:i32
// }

// ama r# onların keyword olmadığını söyleyebiliriz.
#[derive(Debug)]
struct Value {
    r#for: i32,
    r#in: i32,
}

impl Value {
    fn new(r#in: i32, r#for: i32) -> Self {
        Self { r#for, r#in }
    }
}

#[derive(Debug)]
struct Database {
    address: String,
    port: u16,
}

impl Database {
    // Associated function
    fn get_stats() {
        println!("İstatistik bilgileri...");
    }

    pub fn new() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 3456,
        }
    }

    // Method
    pub fn setup(&mut self, address: String, port: u16) {
        self.address = address;
        self.port = port;
    }
}
