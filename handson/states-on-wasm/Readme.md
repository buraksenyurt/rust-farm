# WASM Temelli Oyunlarda State Yönetimi

Çok basit bir oyun bile olsa her oyunda menü, oyunun oynanış ekranı, belki farklı seviyeler, oyuncu yandığı zamanki konum gibi farklı state'ler söz konusudur. Diğer oyun denemelerimde de menüler ve sahne geçişleri söz konusu olduğunda ana oyun motorunda state yönetimini yapmaya çalışıyorum. Bu örnekte çok basit olarak ana menüden asıl oyun sayfasına ve tekrar geriye geçişler ele alınıyor.

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