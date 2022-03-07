# Sağdan Soldan Defterlerime Aldığım Bazı Notlar

Rust dilini öğrenmek ve etkili şekilde kullanabilmek için pek çok kaynaktan yararlanıyorum. Cuma günleri düzenli olarak bülten gönderen [This Week in Rust'tan](https://this-week-in-rust.org/) Amazon'dan aldığım kitaplara, Udemy eğitimlerinden kişisel web sitelerine kadar birçok kaynak...Bazen bu kaynaklardan yakaladığım bilgileri defterlere not aldığımı fark ettim. Deftere not almak o kadar da teknolojik sayılmayan ama etkili bir öğrenme yöntemidir aslında. Bu dokümanda ilgili notları tekrar etmek ve bir düzene koymak açısından oluşturuldu.

- cargo.toml, bir rust projesinin kendisi ve bağımlılıkları hakkında çeşitli bilgiler tutar. Dosyanın içeriği kolayca okunabilir bir formattadır ve yorum satırların eklenmesine de izin verir niteliktedir. TOML'ın açılımı ise [Tom's Obvious, Minimal Language](https://en.wikipedia.org/wiki/TOML) 'tir. Yazarın adı Tom Preston-Werner.
- Rust'ın ilgi çeken yazımlarından birisi **Turbo Fish** olarak adlandırılıyor. **::<>** şeklindeki bu yazımda <> balığı, :: ise o hızla ilerlerken arkasında bıraktığı kabarcıkları ifade etmekte :D
- String::new(); heap'te bir referans açar. Ayrıca String, **Smart Pointer**'dır.
- Ownership kuralları:
  - Bir değer *(value)* tek bir değişken *(variable)* tarafından sahiplenilir.
  - Değişken sahibi scope dışına çıktığında tuttuğu değe yok olur *(deallocate)*
  - Bir t anında sadece tek bir sahip *(owner)* olabilir.
- Double Free, memory corruption'a yol açan bir durumdur. Aynı değere refere eden iki String değişken düşünelim. Scope sonlanırken kurallara göre her ikisi de deallocate edilmeye çalışılır. Bu durum Double Free olarak adlandırılıyor. Rust buna göre bir değerin sahipliğinin tek bir değişkende olmasını garanti eder. Örneğin aşağıdaki input değerinin sahipliği s'de olduğu için derlenmez.

```rust
use std::io;

fn main() {
  let mut input=String::new();
  let mut s=input;  // The ownership of the string is moved to the variable s
  io::stdin().read_lin(&mut input);
}
  ```
- Rust dilinde tüm string'ler UTF8 formatındadır. Dolayısıyla bir karakter veri 1 byte'tan fazla yer tutabilir. Bunun sebebi Unicode kullanımıdır. Örneğin emojiler, japon harfleri. Buna göre aşağıdaki kod farklı çalışır.

```rust
fn main() {
    let emojis=String::from("🍔🍟🍨🍯");
    let slice=&emojis[..4];
    println!("{}",slice); // Sadece 🍔 döner
    let slice=&emojis[..8];
    println!("{}",slice); // 🍔🍟 döner
}
```

- Tüm dosya ve klasörler **module**'dür. Rust projesinin kendisi ise **crate** olarak adlandırılır. Rust proje hiyerarşisinde birden fazla dosya olabilir ki her biri birer module'dür. Ayrıca bu dosyalar klasörler içinde yer alabilir ki bu klasörler de ayrıca module'dür. Dosya veya klasör şeklindeki bir modülü uygulamada kullanmak istediğimizde mod anahtar kelimesini kullanırız. Bazen klasörler içinde gördüğümüz **mod.rs** dosyasının bir kullanım amacı o klasörden public olarak açılacak diğer enstrümanların tanımlandığı yer olmasıdır.
  