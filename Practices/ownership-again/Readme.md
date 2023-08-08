# Mülkiyet(Ownership) Konusu

Rust'ın zor konularından birisi mülkiyet hakku kurallarıdır(Ownership Rules) Kuralın maddeleri bellidir.

- Her değerin bir sahibi vardır _(Each Value has an owner)_
- Bu değerin herhangi bir zamanda yalnızca 1 sahip olabilir. _(There can only be 1 owner at any point in time for that value)_
- Değerin sahibi kapsam dışına çıkınca değeri “düşürülür” _(When the owner goes out out scope, the value will bir dropped)_

## 00 Başlangıç Durumu

İlk olarak aşağıdaki kod parçası ile başlayalım.

```rust
fn main() {
    // #1nci Örnek
    {
        let nick_name = String::from("Dam Van Dam");
        // nick_name ve amount değişkenleri ve sahip oldukları değerler sadece bu scope içinde yaşarlar
        let amount = 1000;
    }
    // Burada nick_name'in ve amount'un sahip olduğu değerler düşürülür
    // ve deallocation yapılır
    println!("Nick name is '{nick_name}' and amount is {amount}");
}
```
nick_name isimli değişken sadece tanımlı olduğu scope içinde yaşar. Bu ko aşağıdaki hata mesajının oluşmasına neden olur?


```text
error[E0425]: cannot find value `nick_name` in this scope
  --> src/main.rs:10:30
   |
10 |     println!("Nick name is '{nick_name}' and amount is {amount}");
   |                              ^^^^^^^^^
   |
help: the binding `nick_name` is available in a different scope in the same function
  --> src/main.rs:4:13
   |
4  |         let nick_name = String::from("Dam Van Dam");
   |             ^^^^^^^^^

error[E0425]: cannot find value `amount` in this scope
  --> src/main.rs:10:57
   |
10 |     println!("Nick name is '{nick_name}' and amount is {amount}");
   |                                                         ^^^^^^
   |
help: the binding `amount` is available in a different scope in the same function
  --> src/main.rs:6:13
   |
6  |         let amount = 1000;
   |             ^^^^^^

For more information about this error, try `rustc --explain E0425`.
error: could not compile `ownership-again` (bin "ownership-again") due to 2 previous errors
```

## 01 Borrow of Moved Value oluşması

Boyutu bilinen ve stack'de duran değerlerin sahipliği ile heap'te duranlar atama işlemlerinde farklı davranışlar gösterirler. Aşağıdaki kod parçasını ele alalım.

```rust
fn main() {
    // 2nci Örnek (Value moved occurs here)
    let point_1 = 10;
    let mut point_2 = point_1; // i32'nin boyutu bellidir, Deep Copy olur. point_2 değeri point_1'e kopyalanır.
    println!("P1={point_1} , P2={point_2}");
    point_2 = 15; // point_2 de yapılan değişiklik point_1'i etkilemez. 10 ve 15 değerlerinin sahipleri farklıdır.
    println!("P1={point_1} , P2={point_2}");

    // Ama aşağıdaki duruma bakarsak
    let my_name = String::from("Clot Van Damme");
    let your_name = my_name.clone(); // Clot Van Damme'ın sahipliği el değiştirir ve your_name'e geçer. my_name artık kullanılamaz.
                                     // Buna göre aşağıdaki kullanım borrow of moved value hatasının doğmasına neden olur
    println!("My name is {my_name} and your name is {your_name}");
    // elbette your_name.clone() ile bu hatanın önüne geçebiliriz.
    // Bu durumda Deep Copy yapılıp her iki değişken de kullanılabilir.
    // Lakin bilindiği üzere deep copy operasyonu pahalıdır.
}
```

Bu durumda aşağıdaki derleme zamanı hatası oluşur.

```text
error[E0382]: borrow of moved value: `my_name`
  --> src/main.rs:22:26
   |
20 |     let my_name = String::from("Clot Van Damme");
   |         ------- move occurs because `my_name` has type `String`, which does not implement the `Copy` trait
21 |     let your_name = my_name; // Clot Van Damme'ın sahipliği el değiştirir ve your_name'e geçer. my_name artık kullanılamaz.
   |                     ------- value moved here
22 |     println!("My name is {my_name} and your name is {your_name}");
   |                          ^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
21 |     let your_name = my_name.clone(); // Clot Van Damme'ın sahipliği el değiştirir ve your_name'e geçer. my_name artık kullanılamaz.
   |                            ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership-again` (bin "ownership-again") due to previous error
```