fn main() {
    // debug trait örneği
    let rust_book = Book {
        id: 1234,
        title: String::from("Web Programming with Rust"),
        in_stock: false,
    };
    println!("{:?}", rust_book); // debug trait olduğunda kullanılabilir
    println!("{:#?}", rust_book); // debug trait davranışının pretty formatta uygulanması

    // struct için Clone trait kullanımı örneği
    // let copy_of_book = rust_book.clone();
    let copy_of_book = rust_book; // Clone davranışı icra edilir
    println!("{:?}", copy_of_book);

    // enum için Copy trait kullanımı örneği
    let color = Colors::Black;
    let carbon = color;
    println!("{:?}", carbon);

    // Default trait kullanımı örneği
    let monthly_report = Report {
        title: String::from("Aylık Marvıl Kahraman Listesi Raporu"),
        max_page: 10,
        ..Default::default()
    };
    println!("{:#?}", monthly_report);
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String, // String içerdiğinden Copy trait'i uygulanamaz. Clone trait'i uygulanır
    pub in_stock: bool,
}

// Copy trait'ini aşağıdaki gibi bir enum'a uygulayabiliriz.
// Lakin Copy trait'i Clone'un alt trait'idir ve onun da bildirilmesi gerekir.
#[derive(Debug, Copy, Clone)]
enum Colors {
    Black,
    // White,
    // Red,
}

#[derive(Debug)]
pub struct Report {
    pub title: String,
    pub max_page: u8,
    pub min_page: u8,
    pub receiver: String,
    pub is_public: bool,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            title: String::from("Status Report"),
            min_page: 1,
            max_page: 3,
            is_public: false,
            receiver: String::from("Department of Justice League"),
        }
    }
}
