# Notlar

Örnekte kullanılacak postgresql veri tabanı için docker imajından yararlanılabilir.

```bash
docker run -p 5433:5432 --name inventory -e POSTGRES_PASSWORD=tiger -d postgres
```

Gerekli paketlerin kurulumu.

```bash
cargo add dotenv
cargo add tokio -F "full"
cargo add sqlx -F "runtime-tokio-rustls","postgres","macros"

# veri tabanı migration işlemlerini kolaylaştırmak için sqlx-cli kullanılmakta
cargo install sqlx-cli --no-default-features -F rustls,postgres

# ilk migration planı aşağıdaki gibi eklenebilir
sqlx migrate add initial

# oluşan sql dosyası içerisinde ilgili tabloları hazırladıktan sonra
# migration planını işletmek için
sqlx migrate run
```