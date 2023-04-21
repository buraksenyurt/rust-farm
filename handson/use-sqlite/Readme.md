# Rust Uygulamalarında SQLite Kullanımı

Bu örnekteki amacım bir Rust uygulamasında SQLite kullanımını deneyimlemek. SQLite hafifsiklet bir veri tabanı olarak küçük boyutta veri kullanan terminal uygulamalarım için ideal görünüyor. SQLite ile Rust tarafındaki iletişim dışında Entity nesneleri ile ilgili işlemler veya migration operasyonları için de Diesel isimli crate'ten yararlanacağım. Örneği Ubuntu 22.04 sisteminde geliştirmekteyim.

```bash
# Sistemde SQLite yüklü olmalı elbette
sudo apt install sqlite3

# sonrasında kontrol için
sqlite3 --version

# diesel ile migration işlemlerini komut satırından yönetmek için
# cli arabirimine ihtiyacımız var
cargo install diesel_cli --no-default-features --features sqlite
```

Örnekte kullanılan crate'ler ise şöyle.

```text
[dependencies]
diesel = { version = "2.0.4", features = ["sqlite", "r2d2"] }
dotenv = "0.15.0"
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
```

## Veritabanı oluşturma ve Migration İşleri

Diesel_cli aracını başarılı şekilde kurduysak aşağıdaki adımlarla devam edip ilk migration planını çalıştırabiliriz. Ancak öncesinde root klasörde .env uzantılı bir dosya açıp içerisine veritabanı bağlantı bilgisini yazmalıyız. Ben veritabanı dosyasını tutmak için Data isimli bir klasör oluşturdum ve .env dosyası içerisinde aşağıdaki içeriği kullandım.

```text
DATABASE_URL=./Data/Steamlopedi.db
```

Migration hazırlıkları için,

```bash
diesel setup
diesel migration generate initial_db
```

Bu komutlar migrations klasöründe tarih bilgisinin kullanıldığı bir klasör oluşturup içerisine up ve down isimli sql dosyalarını bırakır. Buraya yazılan SQL komutları migration upgrate ve downgrade operasyonlarında kullanılır. up.sql ve down.sql dosyalarını tamamladıktan sonra aşağıdaki komut ile migration planı işletilir.

```bash
diesel migration run
```

Terminalden tabloların oluşup oluşmadığını kontrol etmek için aşağıdaki işlemleri yapabiliriz.

```bash
sqlite3 ./Data/Steamlopedi.db
.tables
select * from categories;
.exit
```

![../images/use_sqlite_01.png](../images/use_sqlite_01.png)

Migration başarılı şekilde işledikten sonra tablolar ile ilgili Entity bilgileri de schema.rs içerisinde otomatik olarak oluşur.