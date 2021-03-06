# Rust Programlama Dilinin Temelleri _(Başlangıç Seviyesi)_

Rust dilinin temellerini ve sahip olduğu genel enstrümanların nasıl kullanıldığını öğrenmek amacıyla oluşturulmuş bir dokümandır. Esas itibariyle giriş seviyedeki Rust eğitimlerinde materyal olarak kullanılabilir. Ana hatları ile konu başlıkları şöyledir.

- [x] __Module00:__ Hello World
- [x] __Module01:__ Basit test yazmak, dokümantasyon oluşturmak
- [x] __Module02:__ Temel veri türleri
- [x] __Module03:__ Basit fonksiyon tanımları
- [x] __Module04:__ Module oluşturmak
- [x] __Module05:__ Crate oluşturmak
- [x] __Module06:__ Array veri türü ile çalışmak
- [x] __Module07:__ Vector veri türü ile çalışmak
- [x] __Module08:__ Slice veri türü ile çalışmak
- [x] __Module09:__ Tuple veri türü ile çalışmak
- [x] __Module10:__ Kendi veri türlerimiz için Struct kullanmak
- [x] __Module11:__ Enum veri türü
- [x] __Module12:__ Generic türler
- [x] __Module13:__ Karar yapıları, match ifadesi ve pattern matching
- [x] __Module14:__ Döngüler
- [x] __Module15:__ İlkel haliyle Scope kavramı
- [x] __Module16:__ Closure ifadeleri
- [x] __Module17:__ Higher Order Functions
- [x] __Module18:__ Basit macro kullanımları
- [x] __Module19:__ Trait'ler ile nesnelere davranış kazandırmak
- [x] __Module20:__ Fonksiyonlardan Trait Döndürmek için Box Kullanmak 
- [x] __Module21:__ Built-In Türlere Kendi Trait'lerimizi Eklemek ve Operator Overloading
- [x] __Module22:__ Trait'lerde Static Dispatch 
- [x] __Module23:__ Ownership Kuramı 
- [x] __Module24:__ Borrowing 
- [x] __Module24.5__ Borrowing ve Ownership Konusunu Debugger ile Anlamak
- [x] __Module25:__ Lifetimes 
- [x] __Module26:__ Otomatik Atanan Lifetime Meselesi 
- [x] __Module27:__ Reference Counted Variables 
- [x] __Module28:__ Dosyalarla Çalışmak 
- [x] __Module29:__ Hata Yönetimi (panic, Result<T,Error>, Option<T>)
- [x] __Module30:__ unwrap, expect ve ? operatörü 
- [x] __Module31:__ Threads 
- [x] __Module32:__ Channels
- [x] __Module33:__ Concurrency'de Mutex Kullanımı

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

Trait'leri fonksiyonara parametre olarak geçerken derleyicinin karşısında çözmesi gereken bir durum vardır. Oraya hangi trait uyarlamasını koyacak. Normalde biz bir şey söylemeden bu belli olur. Derleyici static dispatch uygulayarak olası tüm fonksiyon versiyonlarını kendisi yazar. Bu hız kazandırır nitekim derleme zamanında bağlanacak fonksiyonlar hazırdır. Ancak kod tekrar eder ve şişkinlik oluşur. Bu nedenle dinamik dispatch tekniği de uygulanabilir. Generic trait çalışma zamanında bağlanır. Tabii late binding gibi bir durum söz konusu olduğundan bunun kaybı da geç fonksiyon çağrımı olacaktır.

```shell
cargo new traits_dispatch --lib
cargo clippy
cargo test
```

![images/mod22_1.png](images/mod22_1.png)

### Module23: Ownership Kuramı

Rust bellek yönetimi konusunda önemli kavramlardan birisi değişken sahipliğidir. Belleğin belli bir parçasının sahipliği sadece tek bir değişkende olabilir. Bu, data races durumunu engeller ve paralel programlamadaki hataları azaltır. Verinin sahipliğinin taşınması primitive tipler için değerin kopyalanması anlamına gelir. Nitekim primitive tiplerin bellekte kaplayacağı yer bellidir ve kopyalanma maliyeti düşüktür. Ancak structure gibi karmaşık veri tipleri arasında atamalar söz konusu olduğunda sahiplik bir değişkenden diğerine transfer edilir. Bunun sebebi bellekte ne kadar yer tutulacağının derleme zamanında kestirilemeyişidir. Rust bu konuda çok titizdir.

```shell
cargo new ownership
cargo clippy
cargo test
```

vector ataması sonrası oluşan ownership ihlaline ait görüntü.

![images/mod23_1.png](images/mod23_1.png)

Örneğin çalışma zamanındaki durumu.

![images/mod23_2.png](images/mod23_2.png)

### Module24: Borrowing

Bir değişkenin tuttuğu verinin sahipliğinin başka bir değişkene geçici süreliğine verilmesidir. Bu anlamda ownership'ten farklıdır. Ownership'te sahiplik el değiştirdiğinde orjinal değişken geçerliliğini yitirir. Borrowing işleminde & operatörü ile sahiplik referansı alınır, * ile de referans edilen verinin okunur.

```shell
cargo new borrowing
cargo clippy
cargo run
```

Borrowing hatası.

![images/mod24_1.png](images/mod24_1.png)

Vector kullanımında sahiplik into_iter'e geçmesi sonrası oluşan hata durumu.

![images/mod24_2.png](images/mod24_2.png)

Örneğin çalışır hali.

![images/mod24_3.png](images/mod24_3.png)

### Module24.5: Borrowing ve Ownership Konusunu GDB (GNU Debugger) ile Anlamak

Eğitim sırasında öğrencilere sadece izleyecekleri bir örnek şeklinde anlatılabilir. Ubuntu sistemde yüklü olan GDB aracı kullanılır.

```shell
cargo new own_and_borr
cd own_and_borr
cargo run

# Build işlemi yaptıktan sonra debugger kullanılabilir
cargo build
cd target/debug

# Debugger tarafında yapılacaklar
gdb own_and_borr
# borrowing fonksiyonuna breakpoint
b say_what
# ownership fonksiyonuna breakpoint
b whats_say_you
# ilk fonksiyonu çağırdığımız satıra breakpoint
b main.rs:29
# programı çalıştır
r
# main'deki local değişkenlere bak
info locals
# bir sonraki breakpoint noktasına geç
c
# fonksiyon argümanına bak
info args
# argümanın pointer olduğunu gördükten sonra değerini oku
p *input
# sonraki breakpoint noktasına geç
c
# fonksiyona gelen argümana bak
info args
# işleme devam et.
c
# program sonlanır
```

![images/mod24_5_1.png](images/mod24_5_1.png)

### Module25: Lifetimes

Nesnelerin yaşam süreleri vardır. Bunlar çoğunlukla scope'larca belirlenir. Genellikle otomatik olarak atanırlar ama bazı hallerde bizim yaşam ömrünü açıkça belirtmemiz beklenir.

```shell
cargo new lifetimes
cargo clippy
cargo run
```

Örneğin ilk versiyonunda lifetime hatası gösterilmektedir.

![images/mod25_1.png](images/mod25_1.png)

'a ile yaşam ömrü kullanımı sonrası örnek sorunsuz şekilde çalışacaktır.

![images/mod25_2.png](images/mod25_2.png)

### Module26: Otomatik Atanan Lifetime Meselesi

Bazı durumlarda nesnelerin lifetime bilgileri otomatik olarak atanır. Yukarıdaki örnekle karışmaması için yeni bir kodla devam edebiliriz.

```shell
cargo new lifetimes2
cargo clippy
cargo run
```

Gerekmediği halde açıkça lifetime belirttiğimiz durumlar clippy'nin gözünden kaçmayacaktır.

![images/mod26_1.png](images/mod26_1.png)

İç scope'ta bir veriyi ödünç alan ve scope dışında da kullanılmak istenen bir değişken söz konusu olduğunda alınacak hatanın çıktısı.

![images/mod26_2.png](images/mod26_2.png)

Tabii örneği kusurla halde bırakmamak lazım. Düzeltmeler sonrası aşağıdaki gibi çalışmalı.

![images/mod26_3.png](images/mod26_3.png)

### Module27: Reference Counted Variables

Bir struct içinde birden fazla referans türü değişken kullanıldığı durumlara sıklıkla rastlanır. Struct nesnesi farklı scope'larda kullanılıyorsa bu referanslar için ownership veya borrowing kurallarına takılmak pekala kolaydır. Ancak __reference counted variables__ kullanılarak söz konusu ihlallere takılmadan ilerleyebiliriz. __Rc<T>__ kullanarak heap'te yer ayrılmış bir bellek bölgesinin sahipliğinin paylaşılması sağlanabilir.

```shell
cargo new rcv
carg clippy
cargo run
```

![images/mod27_1.png](images/mod27_1.png)

### Module28: Dosyalarla Çalışmak

Dosyaya yazma ve dosyadan içerik okuma işlemlerine kısaca bakalım.

```shell
cargo new ioops --lib
cd ioops
# Dosya append denemeleri için bir dosyaya ihtiyacımız olacak
touch src/games.data

cargo clippy
cargo test
```

![images/mod28_1.png](images/mod28_1.png)

### Module29: Hata Yönetimi (panic, Result<T,Error>, Option<T>)

Rust tarafında iki tür hata söz konusudur. Kurtarılabilir _(Recoverable)_ olanlar ve tam tersi kurtarılamaz _(Unrecoverable)_ olanlar :) Kurtarılabilir hatalar için Resut<T,Err> tipi kullanılır. Diğer türde hatalarda akışın kesintiyi uğraması söz konusudur ve bu durum bir paniğin hortlaması olarak da düşünülebilir.

```shell
cargo new err_handling
cargo clippy
cargo run
```

Vector'ün olmayan bir elemanına ulaşmak istediğimizde panikleyen çalışma zamanı.

![images/mod29_1.png](images/mod29_1.png)

Tipik sıfıra bölme fonksiyonunda bilinçli olarak panic ürettiğimizdeki durum.

![images/mod29_2.png](images/mod29_2.png)

### Module30: unwrap, expect ve ? operatörü

Hata yönetiminde kullanılan önemli enstrümanlar var. unwrap, expect ve ? operatörü. Dönüş türü Result olan fonksiyonlarda match pattern'lerini kısaltman mümkündür.

```shell
cargo new err_handling2
cargo clippy
cargo run
```

pattern matching ile Err durumunu alıp ortamı paniklettiğimiz durum.

![images/mod30_1.png](images/mod30_1.png)

unwrap kullanımındaki çıktı.

![images/mod30_2.png](images/mod30_2.png)

expect kullanımındaki çıktı.

![images/mod30_3.png](images/mod30_3.png)

? olmadan dosya okuması yaptığımız fonksiyon çağrısı için çıktı.

![images/mod30_4.png](images/mod30_4.png)

src/serial.dat dosyası olduğuda ve ? operaötörünü içeren fonksiyonu da kullandığımızdaki çıktı.

![images/mod30_5.png](images/mod30_5.png)

### Module31: Threads

Birden fazla thread ile pek çok işi eş zamanlı olarak işlettirebiliriz. Rust dilinde ownership ve borrowing gibi bellek sahasını güvende tutan mekanizmalar olduğundan ve data race gibi durumların oluşmasına müsaade edilmediğinden thread'ler oldukça etkilidir. Unutmamak gerekir ki bellek üstündeki bir parçanın sadece tek bir sahibi olabilir ki bu durum eş zamanlı çalışan thread'ler için de geçerlidir.

```shell
cargo new threading
cargo clippy
cargo run
```

Main haricinde 4 iş başlattık ve yaklaşık i değeri kadar duraksattık. Geçen toplam süre 4 saniyeler civarında olacaktır. Ve kaç sefer denersek deneyelim thread'ler aynı sırada başlar.

![images/mod31_1.png](images/mod31_1.png)

### Module32: Channels

Bazı hallerde thread'ler arasında haberleşme sağlanması gerekebilir. Örneğin bir thread tarafından tetiklenen bir olay sonrası başka bir thread'in haberdar edilip veri gönderilmesi yoluyla yönlendirilmesi gibi. Thread'ler rust dünyasında güvenli şekilde çalıştırılmaktadır ve aradaki haberleşme kanallar üzerinden icra edilir. Built-In olarak gelen mpsc _(multi-producer single-consumer)_ modülü pek çok temel işlev için yeterlidir. Daha ileri versiyonlarda crossbeam paketi ele alınabilir.

```shell
cargo new channels
cargo clippy
cargo run
```

![images/mod32_1.png](images/mod32_1.png)

### Module33: Concurrency'de Mutex Kullanımı

Mutex, diğer adıyla mutual exclusion thread'ler için bir kilit mekanizmasıdır. Aynı veri üstünde çalışan thread'ler söz konusu olduğunda ortaya çıkabilecek data race durumunu engellemk için kullanılabilir. Bir başka ifadeyle herhangi bir t anında bir veriyi sadece bir thread'in kullanmasını sağlamanın yoludur. Teori basittir. Bir veri bir thread'in sahipliğine geçtiğinde diğer thread'lerin kullanımına kapatılır. Ancak veri bölgesi thread tarafından serbest bırakıldığında başka thread'ler tarafından kullanılabilir. 

```shell
cargo new mutex_sample
cargo clippy
cargo run
```

Programın örnek çıktısı aşağıdaki gibidir.

![images/mod33_1.png](images/mod33_1.png)