# GameStore

Bu örnekte Rocket ve SeaORM küfeleri kullanılarak REST tabanlı bir web api hizmeti örneklenmeye çalışılıyor. Uygulamada bağımsız oyunlar ve bunları geliştiren programcı bilgileri kullanılmakta. Veriyi mysql veritabanında tutmayı planlıyoruz. ORM aracı olarak sea-orm küfesini, REST tarafı servis kabiliyetleri ve çalışma zamanı için de Rocket küfesini kullanacağız.

## Proje Bağımlılıkları

Rust tarafında kullanılan küfelerin yüklenmesi. _(Doğrudan Cargo.toml dosyası içerisine de yazılabilirler)_

```bash
cargo add rocket -F json
cargo add sea-orm -F sqlx-mysql,runtime-async-std-native-tls,macros
cargo add sea-orm-migrator

# migrator işleri için sea-orm-cli kullanılmakta
cargo install sea-orm-cli

# migrator klasörünü ve ilk sürümü oluşturmak için aşağıdaki komut çalıştırılabilir
sea migrate init -d ./src/migrator

# ilk etapta üretilen migration planında gerekli hazırlıklardan sonra
# diğer tablolarla ilgili migration planları da ayrıca hazırlanabilir
# geliştiriciler ile ilgili olan için aşağıdaki yapı kullanılabilir
sea migrate generate -d ./src/migrator create_developer_table

# game tablosu için de aşağıdaki gibi
sea migrate generate -d ./src/migrator create_game_table
```

