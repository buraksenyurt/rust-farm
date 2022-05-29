# Bir Rust Web Assembly Denemesi

Web tabanlı oyun geliştirmede Rust ile Web Assembly işbirlikteliğini öğrenmek için kullanılabilecek bir başlangıç setidir.

## Sistem Hakkında Bilgi

Örnek Ubuntu 20.04 üzerinde geliştiriliyor. rust ortamı kurulu. Ayrıca node.js *(En nihayetinde bir web uygulaması söz konusu)* ve npm de yüklü.

### Proje İskeletinin Oluşturulması

```shell
#1 Proje Klasörü Açılır
mkdir walking-cat
cd walking-cat

#2 Rust Wasm grubunun hazırladığı Rust Webpack şablonu yüklenir
npm init rust-webpack
npm install

#3 Sunucu başlatılır
npm run start

# Sistemde yüklü değilse wasm-pack ile ilgili bir hata alınabilir
# Bu durumda https://rustwasm.github.io/wasm-pack/installer/ adresine gidip
# gerekli kurulum talimatlarını izlemekte yarar vardır
```

