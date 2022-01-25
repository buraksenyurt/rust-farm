# Rust Programlama Dili Temelleri _(Başlangıç Seviyesi)_

Rust dilinin temellerini ve sahip olduğu genel enstrümanların nasıl kullanıldığını öğrenmek amacıyla oluşturulmuş bir dokümandır. Esas itibariyle giriş seviyedeki Rust eğitimlerinde materyal olarak kullanılabilir. Ana hatları ile konu başlıkları şöyledir.

- __Module00:__ Hello World
- __Module01:__ Basit test yazmak, dokümantasyon oluşturmak
- __Module02:__ Temel veri türleri
- __Module03:__ Basit fonksiyon tanımları
- __Module04:__ Module oluşturmak
- __Module05:__ Crate oluşturmak
- __Module06:__ Array veri türü ile çalışmak
- __Module07:__ Vector veri türü ile çalışmak
- __Module08:__ Slice veri türü ile çalışmak
- __Module09:__ Tuple veri türü ile çalışmak
- __Module10:__ Kendi veri türlerimiz için Struct kullanmak
- __Module11:__ Enum veri türü
- __Module12:__ Generic türler
- __Module13:__ Karar yapıları, match ifadesi ve pattern matching
- __Module14:__ Döngüler
- __Module15:__ İlkel haliyle Scope kavramı
- __Module16:__ Closure ifadeleri
- __Module17:__ Higher Order Functions
- __Module18:__ Basit macro kullanımları
- __Module19:__ Trait'ler ile nesnelere davranış kazandırmak
- __Module20:__ Fonksiyonlardan Trait Döndürmek için Box Kullanmak 
- __Module21:__ Built-In Türlere Kendi Trait'lerimizi Eklemek ve Operator Overloading
- __Module22:__ Trait'lerde Static Dispatch 
- __Module23:__ Ownership Kuramı 
- __Module24:__ Borrowing 
- __Module25:__ Lifetimes 
- __Module26:__ Otomatik Atanan Lifetime Meselesi 
- __Module27:__ Reference Counted Variables 
- __Module28:__ Dosyalarla Çalışmak 
- __Module29:__ Hata Yönetimi (panic, Result<T,Error>, Option<T>)
- __Module30:__ unwrap, expect ve ? operatörü 
- __Module31:__ Threads 
- __Module32:__ Channels
- __Module33:__ Concurrency'de Mutex Kullanımı

## Örnekler

Gerekli açıklamalar kod satırları arasında yer almaktadır.

### Module00: Hello World

```shell
# hello_world isimli yeni bir proje oluşturulur.
cargo new hello_world

# çalıştırmak için
cargo run
```

![images/mod00.png](images/mod00.png)

### Module01: newton _(Test yazmak, Doküman Oluşturmak)_

```shell
# Yeni fonksiyonlar taşıyan kütüphaneler oluştururken 
cargo new newton --lib

# Test koşturmak için
cargo test

# Yardım dokümanını oluşturmak için
cargo doc

# sonrasında doc/newton/index.html'e bakılabilir
```

![images/mod01_2.png](images/mod01_2.png)

/// ifadeleri ile oluşturulan yardımcı bilgiler html dosyası üzerinden görüntülenebilir. Tüm Rust çevresindeki standart doküman formatı budur ve markdown tabanlıdır.

![images/mod01_1.png](images/mod01_1.png)

Yardım dokümanları fonksiyon gibi öğelerin kullanımında geliştiriciye önemli ipuçları verir.

![images/mod01_3.png](images/mod01_3.png)

### Module02: scalar_types _(Temel Veri Türleri)_

__Scalar Data Types__

    Integer:

    Size    Signed      Unsigned

    8 bit   i8          u8
    16 bit  i16         u16
    32 bit  i32         u32 (tamsayılar için varsayılan türdür)
    64 bit  i64         u64
    128 bit i128        u128
    arch    isize       usize (mikro işlemci mimarisine göre)

    Float:

    32 bit  f32
    64 bit  f64 (noktalı sayılar için varsayılan türdür)

    Boolean:

    true
    false

    Char:

```shell
# Projeyi oluşturmak için
cargo new scalar_types

# Programı çalıştırmak için
cargo run
```

Çalışma zamanından basit bir görüntü.

![images/mod02_1.png](images/mod02_1.png)

### Module03: Basit Fonksiyonlar _(some_functions)_

Temel fonksiyon kullanımlarına bir bakalım.

```shell
# Kütüphane olarak oluşturalım.
cargo new simple_functions --lib

# test etmek için
cargo test
```

Çalışma zamanına ait örnek görüntü.

![images/mod03_1.png](images/mod03_1.png)

Hemen araya reklam olarak faydalı iki _cargo_ komutu alalım.

```shell
# Kod içeriğini RustFmt'te göre formatlar.
# Yani terminalden de format düzenlemeleri yaptırabiliriz.
cargo fmt

# Bu ise kodun idiomatic olmasını sağlar.
# Yani kodun Rust'ın getirdiği özellikler kullanılarak daha düzgün yazılması için önerilerde bulunur.
cargo clippy
```

Yukarıdaki örnekte yer alan iki fonksiyonu aşağıdaki hale getirelim.

```rust
pub fn greetings(your_name: &str) -> String {
    let message = format!("Merhaba {}.", your_name);
    return message;
}

pub fn square_of_circle(r: f32) -> f32 {
    return PI * r * r;
}
```

Önceki kullanımdan farklı olarak bilinçli bir şekilde return kullandık. Normalde geriye sonuç döndüren fonksiyonlarda son satırda return veya ; kullanımına gerek yoktur. Tail Expression şeklinde ilerlenebilir. _cargo clippy_ komutunu örnek için çalışıtırırsak aşağıdaki uyarıları verir.

![images/mod03_2.png](images/mod03_2.png)

Yani clippy der ki, "kodunu taradım ve aslında daha idiomatic önerilerim var..." Clippy kodu otomatik olarak derler ve belirli problemlerin olup olmadığını tarar. Sonra da sonuçları programcıya tatlı dille aktarır :)

_Clippy pek çok kategoride düzeltmeler sunar. Performas, yazım stili, kod karmaşıklığı, kısıtlar vs...[Detaylar için clippy lint adresine uğramakta yarar var](https://rust-lang.github.io/rust-clippy/master/)_

### Module04: Modül Oluşturma ve Kullanma _(music_shop)_ 

Aynı konu altında birleşen fonksiyonellikleri modül veya sandıklarda _(crate)_ toplarız.

```shell
cargo new soundlib --lib
cd soundlib/src/

# dosya olarak module
touch musicbox.rs
```

### Module05: Crate _(Sandık)_ Kullanımı

Modülleri benzer amaçlar doğrultusunda bir arada toplamanın yolu onları bir sandık _(crate)_ içerisine koymaktır.

```shell
# Örneği aşağıdaki gibi oluşturabiliriz.
cargo new lotary

cd lotary/src/
touch warehouse.rs

# Çalıştırmak için
cargo run
```

Örnekte rastgele sayı üreteci olarak [rand](https://crates.io/crates/rand) isimli crate kullanılmakta. crates.io, rust ile ilgili paketlerin yayınlandığı ortamdır. Buna benzer harici crate bildirimleri toml dosyasında yapılır.

Örnek ilk çalıştırıldığında toml içinde belirtilen harici crate'ler yüklenir.

![images/mod05_1.png](images/mod05_1.png)

Çalışma zamanına ait bir görüntü.

![images/mod05_2.png](images/mod05_2.png)

### Module06: Arrays

Sık kullanılan veri tiplerinden birisi dizilerdir. Diziler sabit boyutludur. Elemanlar aynı tipten oluşur. Dizi elemanlarında değişiklik yapmak için nesnenin mutable tanımlanması gerekir.

```shell
# Örneği kütüphane olarak oluşturalım
cargo new arrays --lib

# Testleri aşağıdaki gibi koşturalım
cargo test
```

![images/mod06_1.png](images/mod06_1.png)

### Module07: Vectors

Rust standart kütüphanesinde yer alan Vec yapısı _(struct)_ ile boyutu değiştirilebilir veri serilerini kullanabiliriz. Vec gibi built-in türlerde rust kütüphanesine geçip ne olduklarına ve nasıl kullanıldıklarına bakmakta yarar var. 
Örneğin Vec tipi için [şuradaki yardım dokümanına](https://doc.rust-lang.org/std/vec/struct.Vec.html#) bakmak ve hatta [src](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#400-403) linkinden yararlanıp nasıl yazıldığını görmekte yarar var. Kullandığımız tüm built-in enstrümanlar için bu şekilde ilerleyerek Rust dilini daha iyi öğrenebiliriz.

```shell
cargo new vectors --lib

cargo test
```

![images/mod07_1.png](images/mod07_1.png)

### Module08: Slice

Bir veri serisinin bellekteki belli bir bloğunu temsil eden işaretçi olarak düşünülebilir. Read-Only'dir ve sabit boyutludur. Boyutu sabit olmasına rağmen çalışma zamanında belirlenebilir. Yaygın olarak Array, Vector ve String türleri ile kullanılır.

```shell
cargo new slices --lib

cargo test
```

![images/mod08_1.png](images/mod08_1.png)

### Module09: Tuples

Tuple farklı türden verileri bir arada tutabilen bir veri türüdür. Statik özelliği taşırlar ve yeniden boyutlandırılamazlar. Dolayısıyla bellek kapsama alanları en baştan bellidir. Elemanları değiştirilebilir, metotlara parametre olarak aktarılabilir veya dönüş tipi şeklinde kullanılabilir.

```shell
cargo new tuples --lib
# Alışkanlık kazanalım yazdıklarımızın idiomatic olup olmadığını görelim.
# Uyarılara kulak verelim ve değiştirelim.
cargo clippy
cargo test
```

![images/mod09_1.png](images/mod09_1.png)

### Module10: Struct Data Type

Kendi veri tiplerimizi struct olarak tanımlarız.

```shell
cargo new structs --lib
cargo clippy
cargo test
```

![images/mod10_1.png](images/mod10_1.png)

### Module11: Enum Veri Türü

Kullanıcı tanımlı veri türlerinden birisidir. Diğer dillerde genellikle sayılara anlamlı isimler vermek için kullanılır. Rust tarafında enum değerleri türlendirilebilir.

```shell
cargo new enums
cargo clippy
cargo run
```

![images/mod11_1.png](images/mod11_1.png)

### Module12: Generics

Kendi veri modellerimizi tasarlarken de generic türlendirmelerden yararlanabiliriz.

```shell
cargo new generics --lib
cargo clippy
cargo test
```

![images/mod12_1.png](images/mod12_1.png)

### Module13: if, match, pattern matching

Pek tabii bu konulara değinmeden olmaz.

```shell
# if,else örneği
cargo new conditions
cargo clippy
cargo run

# match statement örneği
cargo new matching
cargo clippy
cargo run
```

İlk örnekte basit bir sayı tahmin oyunu söz konusu.

![images/mod13_1.png](images/mod13_1.png)

İkinci örnekte çeşitli pattern matching kullanımları var.

![images/mod13_2.png](images/mod13_2.png)

### Module14: Loops

for, while ve loop döngülerinin basit kullanımlarına yer veriliyor.

```shell
cargo new loooops --lib
cargo clippy
cargo test
```

![images/mod14_1.png](images/mod14_1.png)

### Module15: Scope Hakkında Minik Bilgi

Sahip olduğu önleyici tedbir ve kurallar sayesinde Rust, memory leak oluşmasına izin vermez. Bunun en büyük sebeplerinden birisi de değişkenleri elle kaldırmamıza gerek olmamasıdır. Lakin yönetimli dillerde olduğu gibi bir gargabe collector mekanizması da yoktur. Değişkenleri dahil olduğu scope kavramı bu noktada öne çıkar. Scope bittiğinde bu kapsama dahil olan değişkenler derhal yok edilir ve bellek bölgesi boşaltılır. Bunu anlamak için basit bir kod parçamız var.

```shell
cargo new scopes
cargo clippy
cargo run
```

value değişkeni ile ilgili scope hatası.

![images/mod15_1.png](images/mod15_1.png)

global değişken kullanırken scope ihlali söz konusu olabilir ve bu güvensizdir. O nedenle aşağıdaki gibi bir derleme hatası alınır.

![images/mod15_2.png](images/mod15_2.png)

### Module16: Closures

Fonksiyonel dillerinin karakteristik özelliklerinden birisi de closure'dur. Onları isimsiz fonksiyon _(anonymous function)_, lambda, lambda ifadesi veya fonksiyon içinde fonksiyon olarak da biliriz. Closure ifadeleri ile fonksiyonları değişkenlere atamamız da mümkündür.

```shell
cargo new closures
cargo clippy
cargo run
```

generic parametre kullanımında dikkat edilmelidir.

![images/mod16_1.png](images/mod16_1.png)

çalışma zamanı sonucu

![images/mod16_2.png](images/mod16_2.png)

### Module17: Higher Order Functions

Başka bir fonksiyonu parametre olarak alan fonksiyonlar Higher Order Function olarak adlandırılırlar. Fonksiyonel dillerin olmazsa olmaz kabiliyetlerindedir.

```shell
cargo new hofs --lib
cargo clippy
cargo test
```

![images/mod17_1.png](images/mod17_1.png)

### Module18: Macros

Rust öğrenirken sıklıkla println!, vec!, write! gibi fonksiyonlarla çalışırız. Bunlar birer macro'dur. Sondaki ! işaretinden anlamak mümkün. Tabii geniş bir konudur nitekim meta programlamada da kullanılırlar. Macro'ları kod yazan kodlar olarak düşünebiliriz. Bir kod ifadesini _(expression)_ algılayıp üzerinde çeşitli operasyonlar işletebilmemizi sağlarlar. Bu sayede derleme sırasında araya girip kodu genişletmek de mümkün olabilir. 101 seviyesi eğitimi için ileri seviye bir konudur. O nedenle çok basit anlamda nasıl yazıldığına bakılabilir.

```shell
cargo new macros
cargo clippy
cargo run
```

Macrolarda kullanılabilen designator türleri; expr _(expresion)_ ,ident _(variable, function names)_ ,block, stmt _(statement)_ ,pat _(patter)_ ,path,meta,ty _(type)_ ,tt _(token tree)_, vis _(visibility qualifier)_, literal... Kısacası bir kodun temel semantik yapılarını macro içerisinde tespit etmemiz mümkündür. 

![images/mod18_1.png](images/mod18_1.png)

### Module19: Traits

Trait'ler nesne yönelimli dillerdeki interface veya abstract class tiplerine benzetilebilir. Nesne yapılarına yeni davranış modellerini monte etmek için yaygın olarak kullanılırlar. Örnekte çok basit anlamda trait nasıl tanımlanır ve kullanılır anlamaya çalışıyoruz.

```shell
cargo new traits --lib
cargo clippy
cargo test
```

generic say_something fonksiyonuna Entity trait'ini uygulamayan bir tip yollarsak.

![images/mod19_1.png](images/mod19_1.png)

Normal şartlarda ise örnek sorunsuz çalışır.

![images/mod19_2.png](images/mod19_2.png)

### Module20: Fonksiyonlardan Trait Döndürmek için Box Kullanmak

Rust, fonksiyonların geriye döndürdüğü değerler varsa bunların bellekte ne kadar yer kaplayacağını önceden bilmek ister. Bu nedenle bir fonksiyondan trait döndürülmesi ilk etapta mümkün görünmez. Lakin Box yapısını kullanarak _(ki kendisi heap üstünde ayarlanan yer için bir işaretçidir)_ bu fonksiyonelliği sağlayabiliriz.

```shell
cargo new traits_box --lib
cargo clippy
cargo test
```

Örneğin yazdığımız ilk versiyonunda Actor trait'ini fonksiyondan döndürmek istiyoruz. Bu durumda derleyici aşağıdaki hatayı verecektir.

![images/mod20_1.png](images/mod20_1.png)

Box nesnesini devreye alıp birkaç düzeltme yaptıktan sonra.

![images/mod20_2.png](images/mod20_2.png)

### Module21: Built-In Türlere Kendi Trait'lerimizi Eklemek ve Operator Overloading

Rust'ın built-in veri türlerine kendi trait'lerimizi uyarlayabilir ve böylece yeni davranışlar kazandırabiliriz. Hatta bu yaklaşım ile operatörlerin bizim istediğimiz şekilde çalışması da sağlanabilir _(ki bu Operator Overloading olarak bildiğimiz olaydır)_ Operatörler de birer trait'tir aslında. Ooverload edilebilir operatörler için [şu adrese bakılabilir](https://doc.rust-lang.org/stable/core/ops/#structs).

```shell
cargo new traits_4_all --lib
cargo clippy
cargo test
```

![images/mod21_1.png](images/mod21_1.png)

### Module22: Trait'lerde Static Dispatch

__todo!();__

### Module23: Ownership Kuramı

__todo!();__

### Module24: Borrowing

__todo!();__

### Module25: Lifetimes

__todo!();__

### Module26: Otomatik Atanan Lifetime Meselesi

__todo!();__

### Module27: Reference Counted Variables

__todo!();__

### Module28: Dosyalarla Çalışmak

__todo!();__

### Module29: Hata Yönetimi (panic, Result<T,Error>, Option<T>)

__todo!();__

### Module30: unwrap, expect ve ? operatörü

__todo!();__

### Module31: Threads

__todo!();__

### Module32: Channels

__todo!();__

### Module33: Concurrency'de Mutex Kullanımı

__todo!();__