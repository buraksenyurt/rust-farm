fn main() {
    // debug trait örneği
    let rust_book = Book {
        id: 1234,
        title: String::from("Web Programming with Rust"),
        in_stock: false,
    };
    println!("{:?}", rust_book); // debug trait olduğunda kullanılabilir
    println!("{:#?}", rust_book); // debug trait davranışının pretty formatta uygulanması
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String, // String içerdiğinden Copy trait'i uygulanamaz. Clone trait'i uygulanır
    pub in_stock: bool,
}
