# Can-Ban Board

Bildiğimiz Kanban board'u Rust, Wasm ve HTML üçlemesinde geliştirmek istesem nasıl yaparım diye yola çıktığım örnektir.

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
