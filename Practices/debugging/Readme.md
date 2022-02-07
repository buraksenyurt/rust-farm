# GDB ile Debug İşlemleri

Rust ile ilgili öğretilerde bellek yönetimi konusunu incelerken fonksiyon ve değişkenlere ait kapsamların bellekte nasıl açıldığını görmek için GNU Debugger'dan yararlanıldığını gördüm. Oldukça güzel bir araç. Kodu debug etmek deyince insanın aklına Visual Studio gibi gelişmiş IDE'lerin kolaylıkları geliyor ve bu nedenle GDB ilkel bir araç olarak görünüyor. Yine de rust kodunu terminalden adım adım işletmek ve bellek üzerindeki konumlandırmaları görmek adına son derece faydalı bir araç. Rust dilinde ilerleyeceksem onu da tanımam gerekiyor. Öncelikle üzerine çalıştığım Ubuntu platformuna yüklemem lazım. Bu arada GDB ile ilgili detaylar için [şu adrese](http://www.gdbtutorial.com/) bakılabilir.

```shell
sudo apt-get update
sudo apt-get install gdb

gdb --version
```

Eğer her şey yolunda giderse aşağıdaki görüntüde olduğu gibi versiyon numarasını görebilmeliyiz.

![../images/debugging_1.png](../images/debugging_1.png)

GDB kullanımını deneyimlemek için ilk olarak basit bir örnekle ilerleyebiliriz.

```shell
cargo new debugging
cd debugging
cargo clippy
cargo run

# kodun çalıştığından emin olduktan sonra build etmeliyiz
cargo build
```

Gelelim GDB komutları ile bu kod parçasını nasıl debug edeceğimize. Öncelikle build işlemi sonrası oluşan target/debug klasörüne geçelim.

```shell
# Programa ait binary'yi debug modda açalım
gdb debugging
# Çalıştığını görelim
run
# ve ilk satırından itibaren kod içeriğine bir bakalım
list

# ardından örneğin increase_level ve decrease_level fonksiyonlarına birer breakpoint koyalım
b increase_level
b decrease_level

# kodu çalıştıralım
r

# Artık breakpoint noktalarında bir takım bilgilere bakabiliriz.
# Örneğin o andaki local değişkenlere ve argümanlara bakalım
info locals
info args

# Kodu bir adım ilerletelim
n

# Aynı bilgilere tekrar bakalım ve hatta stack bellek bölgesine bir göz atalım.
bt
info locals
info args

# Hatta pointer olarak gelen değişkenlerin içeriklerini şöyle görebiliriz
print *p

# Bir sonraki breakpoint noktasına geçmek için c komutunu kullanırız
c

# stack üzerindeki scope'ları görmek için yine bt'den yararlanabiliriz
bt

# debugger'dan çıkmak içinse aşağıdaki komutu kullanırız.
q

# Bu arada minik bir ipucu bırakalım. Ekran çok kalabalıklaştığında
# muhtemelen silmek isteyeceksiniz. Ctrl + L işinizi görecektir.
```

Tabii bu komutları denerken ekran görüntüsü aşağıya doğru uzayıp gitti. Neyeseki sağdaki dikey monitör biraz olsun işi kotardı. Yine de sonuçları iki parça halinde paylaşacağım.

İlk kısımda gdb'yi açıp kodun içeriğini gösteriyoruz.

![../images/debugging_2.png](../images/debugging_2.png)

Devam eden kısımda ise komutların sonuçlarını görmekteyiz.

![../images/debugging_3.png](../images/debugging_3.png)

Bu kısmı yorumlamak oldukça önemli. Kodumuzdaki fonksiyonlar Player verisini referans olarak ödünç alıp kullanmaktalar. Bu nedenle girdiğimiz fonksiyonlarda birer pointer görmekteyiz. Pointer adresi yazdığımız kod düşünüldüğünde değişmiyor elbette. Dikkat çekici bir diğer nokta fonksiyonlara parametre olarak gelen Player nesnesinin işaret ettiği veri yapısı. Dikkat edileceği üzere String olarak tasarladığımız name değişkeni String veri yapısının tasarımı gereği heap bölgesindeki içeriği işaret etmekte.

GDB aracını kullanarak özellikle Smart Pointer gibi enstrümanların işleyişini anlamak da oldukça kolay. Örnek koda bu aşamada aşağıdaki fonksiyonu eklediğimizi düşünelim.

```rust
fn change_level(p: &mut Player) {
    let level = Box::new(90);
    p.level = *level;
}
```

Sadece konuyu değerlendirmek için level isimli bir smart pointer kullanıyoruz. Smart Pointer'lar scope sonlandığında otomatik olarak heap'ten atılırlar ki bu özellikle silmeyi unuttuğumuz pointer'ların oluşturacağı Memory Leak durumunun oluşmamasını garanti eder. Gerçekten böyle olup olmadığını anlamanın bir yolu kodu debug ederken fonksiyon çağrısı tamamlandıktan sonraki duruma bakmaktır. Öyleyse tekrar...

```bash
gdb debugging
# breakpoint'i ekleyelim
b change_level
# programı çalıştıralım(run)
r
# Birkaç satır ilerleyelim
n
n
n
# change_level fonksiyonu içinde tanımlanan local değişkenlere bir bakalım
info locals
#pointer değerini okuyalım (Tabii siz denerken adres farklı olacaktır)
x /d 0x5555555a5af0

# Kodu ilerletip scope'u sonlandıralım. Yani fonksiyon işleyişini tamamlayalım.
n
n
# Şimdi tekrar aşağıdaki komutu çalıştıralım
x /d 0x5555555a5af0

# sonuç 0 olmalı. Bu Smart Pointer'ın söylediği üzere ilgili bellek bölgesinin kaldırıldığı anlamına gelir.
```

Çalışma zamanı sonuçları aşağıdaki gibidir.

![../images/debugging_4.png](../images/debugging_4.png)

Tabii kalabalık kod parçalarında GDB ile debug işlemleri pek kolay olmayabilir. Hatta sağlıklı loglar daha çok işe yarayabilir. Yine de iç dinamikleri öğrenme aşamasındayken bu debugger'ı kullanmak bence oldukça önemli.