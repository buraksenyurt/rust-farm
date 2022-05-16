# Sauron ile Basit Bir Web Uygulaması Yazalım

Sauron, Web Assembly ile çalışır. Sistemde rust haricinde wasm-pack yüklü olmalı nitekim örnekte rust kodlarının wasm'a çevrilmesi söz konusu. Ayrıca statik dosyaların daha kolay sunulması için [basic-http-server](https://crates.io/crates/basic-http-server) küfesi kullanılıyor.

```shell
# wasm-pack için gerekli bazı binary'ler için
sudo apt install build-essential libssl-dev pkg-config ca-certificates

# wasm-pack kurulumu 
cargo install wasm-pack

# basic-http-server kurulumu
cargo install basic-http-server

# projenin oluşturulması
cargo new --lib hello_sauron
```

