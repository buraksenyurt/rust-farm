# OOP ama Performans Odaklı

Bu örnekte aynı davranışları olan 3 farklı veri modelinin yüksek sayıda örneği üstünden hesaplama yapmak istediğimiz de oluşabilecek performans farklılıkları ele alınıyor. Senaryo da uçak, topçu bataryası ve tank gibi oyun karakterlerini ifade eden basit veri yapıları var. Bazı özelliklerine göre bunların yakıt maliyetlerini hesaplayan ortak fonksiyonellikler söz konusu. Amaç ortada bu nesnelerin milyonlarcası varken tamamının toplam yakıt maliyetini bulmak.

Kod tarafında bunun için version_00, version_01 ve version_02 isimli kod parçları mevcut. İlk versiyon ortak davranışların toplu koleksiyonlar üstünde işletilmesi için trait ve Box smart pointer'ından yararlanıyor. İkinci ve daha performanslı çalışan örnek ise trait ve Box yerine saf enum veri modelini kullanıyor. Son modelde ise tüm entity bileşenlerine ait listeleri taşıyan tek bir veri modeli var.

Not: Bir sebepten son versiyondaki toplam değer diğerlerinden farklı çıkmakta. Bunun sebebini bulmak lazım.

```bash
# farklı versiyonları aynı projede kullanabilmek için
# toml dosyasına [[bin]] kısımları eklendir.

# ilk versiyonu çalıştırmak için
cargo run --bin version_00

# ikinci versiyonu çalıştırmak için
cargo run --bin version_01

# üçüncü versiyonu çalıştırma için
cargo run --bin version_02
```