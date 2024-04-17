# Bir Başka Rust WASM Denemesi

Bu örnekte HTML 5'in SVG elementinden de yararlanarak sayfadaki bir dörtgenin yön tuşları ile hareket ettirilmesine bakıyoruz. Dörtgenin temel özelliklerini içeren veri modelini ve hareket sistemini Rust tarafında ele almaktayız. 

```bash
# wasm pack kurulumu için
cargo install wasm-pack

# WASM paketini hazırlamak için
wasm-pack build --target web
```

## Sunucu Yayınlama

Sayfayı dilersek basit bir node sunucusu üzerinden yayınlayabliriz. Bunun için aşağıdaki komutlarla ilerlenebilir.

```bash
# root klasördeyken
touch server.js

npm init -y

# Express modülü host işlemlerimizi kolaylaştırır
npm install express

# Örneği çalıştırmak için, yine root klasördeyken
npm start
```