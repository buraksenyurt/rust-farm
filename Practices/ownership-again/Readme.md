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

## 01 move Oluşması

Boyutu bilinen ve stack'de duran değerlerin sahipliği ile heap'te duranlar atama işlemlerinde farklı davranışlar gösterirler. Aşağıdaki kod parçasını ele alalım.

```rust

```