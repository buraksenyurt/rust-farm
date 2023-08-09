# C++ vs Rust

Rust'ın ownership ve borrow checker mekanizmaları C veya C++'ta sıklıkla yapabileceğimiz bazı bellek hatalarının henüz derleme aşamasında kesilmesini sağlıyor. Buradaki çalışmada bu farkları incelemeye çalışıyorum.

## Case 01 : Use After Frees

Bellekten silinmiş bir değerin referansına silme işleminden sonra tekrardan erişmeye çalışmak. Bunun için use_after_frees.cpp isimli dosyayı derleyip çalıştıralım.

```shell
# derlemek için
gcc use_after_frees.cpp -lstdc++ -o use_after_frees

# derlenen kodu çalıştırmak için
./use_after_frees
```

C++ ile yazılan kodda Use After Free durumunu simüle etmek için delete işleminden yararlanılmıştır. delete ile nesnenin bellekten silinmesi işlemi açıkça gerçekleştirilmiş olur. Ancak C++ kodu başarılı bir şekilde derlenir. Çalışma zamanında ise hata verir.

```text
terminate called after throwing an instance of 'std::length_error'
  what():  basic_string::_M_create
Aborted (core dumped)
```

bazen de şöye bir çalışma zamanı hatası oluşur.

```text
terminate called after throwing an instance of 'std::bad_alloc'
  what():  std::bad_alloc
Aborted (core dumped)
```

Rust ile yazdığımız ve Scope dışını çıkıldığında nesne deallocate örneğinin sergilendiği durumda ise kod derleme zamanında hata verir.

```text
error[E0382]: borrow of moved value: `player`
  --> src/main.rs:24:24
   |
21 |     let player = Player::new("Champion".to_string(), 1000);
   |         ------ move occurs because `player` has type `Player`, which does not implement the `Copy` trait
22 |     delete(player);
   |            ------ value moved here
23 |     // Ownership kuralları gereği derleme zamanında hata alınır
24 |     let player_title = player.get_title(); // borrow of moved value: `player`
   |                        ^^^^^^^^^^^^^^^^^^ value borrowed here after move
   |
note: consider changing this parameter type in function `delete` to borrow instead if owning the value isn't necessary
  --> src/main.rs:29:19
   |
29 | fn delete(player: Player) {
   |    ------         ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function

For more information about this error, try `rustc --explain E0382`.
error: could not compile `case_01` (bin "case_01") due to previous error
```