# Rust Uygulamalarında SQLite Kullanımı

Bu örnekteki amacım bir Rust uygulamasında SQLite kullanımını deneyimlemek. SQLite hafifsiklet bir veri tabanı olarak küçük boyutta veri kullanan terminal uygulamalarım için ideal görünüyor. SQLite ile Rust tarafındaki iletişim dışında Entity nesneleri ile ilgili işlemler veya migration operasyonları için de Diesel isimli crate'ten yararlanacağım. Örneği Ubuntu 22.04 sisteminde geliştirmekteyim.

```bash
# Sistemde SQLite yüklü olmalı elbette
sudo apt install sqlite3

# sonrasında kontrol için
sqlite3 --version

# Uygulamada kullanılan crate'ler aşağıdaki gibi yükleyebiliriz
# Web API çalışma zamanı ve ortamı için actix-web, actix-rt
# Connection Pool desteği için R2d2
# SQLite tarafı, persistance, ORM ve migration işleri için Diesel
# çevre değişkenlerinin yönetimi için (.env dosyası) dotenv
# serileşme operasyonları için de serde
# idiomatic hata yönetimi desteği için anyhow
# zaman verileri için chrono
cargo add actix-web actix-rt dotenv serde serde_derive serde_json anyhow chrono
cargo add diesel -F "sqlite","r2d2"

# diesel ile migration işlemlerini komut satırından yönetmek için
# cli arabirimine ihtiyacımız var
cargo install diesel_cli --no-default-features --features sqlite
```