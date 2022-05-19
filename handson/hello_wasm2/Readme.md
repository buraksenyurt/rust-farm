# WASM Çıktısını Web Sunucusu ile Birlikte Çalıştırmak

**hello_wasm** isimli çalışmada index.html içerisinde wasm binary'sini çalıştırmıştık. Ama index.html'i fiziki lokasyonundan işletmiştik. Pek tabii normalde bu sayfaların bir web sunucusu üzerinde olması ve işletilmesi gerekiyor. Bu örnekte nodejs ve webpack araçlarını kullanarak ilerleyeceğiz.

```shell
# İlk hazırlıklar
# Sistemde nodejs ve npm'in yüklü olduğunu varsayıyoruz

# Projenin oluşturulması
cargo new --lib hello_wasm2
cd hello_wasm2
mkdir www

# hello_wasm örneğinde kullandığımız calc.wasm dosyası www altına taşınır

# www klöasrüne geçilir
cd www

# package.json oluşturulur
npm init -y
```