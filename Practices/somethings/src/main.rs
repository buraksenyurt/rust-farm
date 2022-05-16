fn main() {
    // Raw Identifier Örneği. Keyword'lerin field isimlerinde kullanılabilmesi.
    let x = Value::new(10, 10);
    println!("{:?}", x);
    println!("{:#?}", x);
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
