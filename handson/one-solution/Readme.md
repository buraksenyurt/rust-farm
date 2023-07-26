# Rust ile Bir Proje İskeletinin Oluşturulması

Microsoft .Net platformunda çalışanlar genellikle bir Solution ve altında toplanmış projelere aşinadır. Rust her ne kadar sistem programlama dili odaklı olsa da zaman zaman birden fazla rust uygulamasının bir arada yer aldığı siloların geliştirilmesi de söz konusudur. Özellikle servis odaklı yaklaşımlarda bunu sıklıkla görebiliriz. İşte bu çalışmadaki amaç bir Rust projeler topluluğunun tek çatı altında nasıl toplanabileceğini incelemektir.

## Seyrüsefer

Yıldız tarihi 2023. Aynı workspace içerisine dahil olması planlanan rust projeleri için aşağıdaki işlemler yapılır.

```shell
# Root klasörü açıldı
mkdir one-solution
# İçerisine girilip Cargo.toml dosyası eklendi
# ki projeler ve ortak crate'leri içerecek
cd one-solution
touch Cargo.toml

# Örnek cargo kütüphaneleri ve uygulamaları açıldı

# veritabanı operasyonları
cargo new --lib almanac_data
# rabbitmq kuyruk işlemleri
cargo new --lib almanac_queue
# ortak kullanımlar
cargo new --lib almanac_common
# servis uygulaması
cargo new almanac_api
# analitik uygulaması
cargo new almanac_analytic
```

Projelerin açılması sonrası root klasörde yer alan cargo.toml dosyası da buna göre düzenlenmelidir. Bir Workspace oluşturulduğunda klasördeki hangi projelerin buraya dahil olduğu cargo.toml dosyasındaki workspace alanında belirtilir. Benzer şekilde tüm projelerin ortaklaşa kullanacağı crate'ler de buradaki workspace.Dependecies kısmına yazılır. İlk etapta root klasördeki cargo.toml dosyası aşağıdaki gibidir.

```toml
[workspace]
members = ["almanac_analytic","almanac_api","almanac_common","almanac_data","almanac_queue"]

[workspace.dependencies]
tokio = {version = "1.29.1", features = ["full"]}
serde = {version = "1.0.174", features = ["derive"]}
serde_json = "1.0.103"
thiserror = "1.0.44"
axum = "0.6.19"
tracing = "0.1.38"
tracing-subscriber = { version = "0.3.17", features = ["json","time","env-filter"]}
```

Şimdilik ihtiyaç duyulabilecek bazı küfeler eklendi. Asenkron operasyonlar için tokio, rest api tarafı için axum, hata yönetimini kolaylaştırmak için thiserror,yapılsal olay günlükleri için tracing, burayla etkileşim için tracing-subscriber, json serileştirme tarafları için serde ve serde_json gibi. İhtiyaç duydukça başka crate'ler de buraya eklenebilir ama genel itibariyle workspace'in oluşturulmasının esasları buradaki gibidir.