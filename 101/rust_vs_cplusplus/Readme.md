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
21 |     let player = Player::new("Lorna".to_string(), 55);
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

## Case 02 : Double Frees

Serbest bırakılmış bir bellek bölgesini tekrardan serbest bırakmaya çalışmak. Bununla ilgili double_frees.cpp programını göz önüne alabiliriz.

```shell
gcc double_frees.cpp -lstdc++ -o double_frees
./double_frees
```

Kod başarılı şekilde derlenir ancak çalışma zamanında Segmentation Fault hatası alınır. **Double Free**, aynı bellek alanını iki kez serbest bırakmaya çalıştığında oluşur. Koddaki ilk delete işlemi başarılı bir şekilde belleği serbest bırakır ve daha sonra aynı bellek alanı tekrar serbest bırakmaya çalışır. Bu çalışma zamanında **Segmentation Fault** hatasına neden olur.

Benzer bir kodu Rust ile yazdığımızda ise derleme zamanında hata alırız.

```text
error[E0382]: use of moved value: `player`
  --> src/main.rs:16:18
   |
13 |     let player = Player::new("Dolares".to_string(), 67);
   |         ------ move occurs because `player` has type `Player`, which does not implement the `Copy` trait
14 |     do_something(player);
   |                  ------ value moved here
15 |
16 |     do_something(player);
   |                  ^^^^^^ value used here after move
   |
note: consider changing this parameter type in function `do_something` to borrow instead if owning the value isn't necessary
  --> src/main.rs:19:25
   |
19 | fn do_something(player: Player) {
   |    ------------         ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function

For more information about this error, try `rustc --explain E0382`.
error: could not compile `case_02` (bin "case_02") due to previous error
```

## Case_03 Buffer Overflow

Genellikle belli boyuttaki bir dizinin index dışındaki bir alanına erişip değer atamaya çalıştığımızda karşılaştığımız türden bir hata olduğunu söyleyebiliriz. buffer_overflow.cpp isimli C++ kodunda bu durum ele alınır. 

```shell
gcc buffer_overflow.cpp -lstdc++ -o buffer_overflow
./buffer_overflow
```

Burada 10 elemanlı bir tam sayı dizisinin 11nci elemanına ulaşılmaya çalışılır. C++ çalışma zamanı aşağıdaki hatayı döndürür.

```text
someData = 1
*** stack smashing detected ***: terminated
Aborted (core dumped)
```

Dikkat edileceği üzere someData değeri ekrana yazdırılmıştır. Yani kodun belli bir kısmı çalışmıştır ancak dizi sınırlarını taştığımız durum için **stack smashing detected** şeklinde bir hata dönmüştür. Rust'ın bu duruma yaklaşımı ise şöyledir. 

```text
thread 'main' panicked at 'index out of bounds: the len is 10 but the index is 10', src/main.rs:6:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Tabii Rust içinde derleme zamanında yakalanan bir hata durumu söz konusu değildir ancak kodun hiçbir bölümü çalıştırılmayıp geliştirici için daha anlamlı bir hata mesajı verilmektedir.

## Notlar

- **Buffer Overflow** hatası ile ilgili olarak kaynaklarda geçen bir solucan var. Morris solucanı olarak bilinen saldırı sonrası solucan 1988'de Internetteki 60bin makineye bulaşmış. Internetin büyük bölümü birkaç günlüğüne kapanmış. Morris, bazı Unix sistemlerindeki buffer overflow açığını kullanmış. Ama nasıl kullanmış henüz aklım almadı :D
- **Double Free** ile ilgili bazı saldırılar.
  - **2011** yılında **Apache** sunucularında **Range** başlığını içeren isteklerdeki Double Free hatası tetiklenerek saldırılar gerçekleştirilmiş. Saldırı apache kullanan sunucuların devre dışı kalmasına neden oluyordu. Hizmet kesintisi yaşayan sunucular oldu. **Apache Killer**.
  - **2008** yılında **Windows** işletim sisteminin **RPC** işlevindeki Double Free hatası istismar edilerek saldırılar gerçekleştirilmiş. **MS08-067**.
- **Use After Free** ile ilgili bazı saldırılar.
  - **2014** yılında **OpenSSL** kütüphanesindeki Use After Free açığı istismar edilerek saldırılar gerçekleşmiş. SSL/TLS kullanan birçok web sitesi bundan etkilendi ve kullanıcıların özel bilgilerine ulaşılabildiği ortaya çıktı. **Heartbleed**.
  - **2010** yılında ise Windows işletim sistemini hedef alan ve Use After Frees hatasını kullanan **Stuxnet** isimli bir solucan peydahlandı. Bu solucan İran'ın nükleer santral santrifüjünün zarar görmesine neden olmuş. Böylece endüstriyel kontrol sistemlerinin siber saldırılardan nasıl etkilenebileceği de görülmüş oldu.