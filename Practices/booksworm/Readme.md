# BooksWorm

Kütüphanedeki kitapları girmek için kullanabileceğimiz basit bir formu Rust, WebAssembly ve Javascript kullanarak nasıl geliştirebiliriz? Bu kodlama pratiğinde bu soruyu cevaplamaya çalışıyoruz.

## İlk Hazırlıklar

```shell
# Proje iskeletinin oluşturulması.
cargo new --lib booksworm
cd booksworm

mkdir www
cd www

npm init -y

npm install --save webpack webpack-cli
npm install --save-dev webpack webpack-dev-server
npm install --save copy-webpack-plugin

# www klasörüne bir .gitignore dosyası eklenir.
# node_modules klasörü github tarafına gitmesin diye düzenlenir

touch bootstrap.js
touch index.js
touch index.html
mkdir public
touch webpack.config.js

cd ..
cargo install wasm-pack

# Örneği çalıştırmak
# root klasöründeyken wasm-pack aşağıdaki gibi build edilir
wasm-pack build --target web

# www klasöründeykende
npm run dev
```