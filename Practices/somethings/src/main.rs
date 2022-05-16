fn main() {
    // Raw Identifier Örneği. Keyword'lerin field isimlerinde kullanılabilmesi.
    let x = Value::new(10, 10);
    println!("{:?}", x);
    println!("{:#?}", x);
    println!("{} {}", x.r#in, x.r#for);

    // i32 scalar tipi için atamalar ve bellek adresleri
    let number1 = 32;
    let number2 = &number1;
    let number3 = &number2;

    println!("number1 value {} and address {:p}", number1, &number1);
    println!("number2 value {} and address {:p}", number2, number2);
    println!("number3 value {} and address {:p}", number3, number3);

    let number4 = &&23;
    println!("number4 address {:p}", &&number4);
    println!("number4 address {:p}", number4);
    println!("number4 value {}", number4);

    println!("x address {:p}",&x);
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
