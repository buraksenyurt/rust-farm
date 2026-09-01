# Minik Çaplı Bir Key-Value Store

Bir başka rust pratiği. TCP tabanlı, asenkron çalışan, tamamen string key-value çiftleri taşıyan, dockerize edilebilen
bir key-value store.

## Docker

Dockerize işlemleri.

```bash
# Docker build
docker build -t kivi-store .

# Docker Compose ile Çalıştırma
docker-compose up -d
```

## Genel Kullanım

Sunucuyu doğrudan `cargo` ile çalıştırabilirsiniz:

```bash
cargo run
```

Varsayılan olarak `.env` dosyasındaki `LISTEN_ADDRESS` (`0.0.0.0:5555`) adresini dinler.

Desteklenen komutlar, boşlukla ayrılmış metin satırları şeklinde gönderilir:

```bash
SET key value
GET key
REMOVE key
LIST
```

### İstemci ile Bağlanmak

Projeyle birlikte gelen basit, cross-platform komut satırı istemcisini kullanabilirsiniz:

```bash
cargo run --bin client
```
