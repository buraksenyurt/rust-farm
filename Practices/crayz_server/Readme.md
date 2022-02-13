# Crayz Server _(Sıfırdan Bir HTTP Server Yazmak)_

Aslında HTTP server'lar için pek çok hazır çatı zaten var ancak HTTP taleplerini dinleyen bir sunucuyu Rust ile sıfırdan yazmak bireysel anlamda çok şey öğretecektir. Nitekim gelen HTTP isteklerini dinleyecek örnek TCP/IP katmanında gelen binary mesajları anlamlı HTTP ifadelerine dönüştürmeli, anlamlı HTTP ifadeleri bir mekanizma tarafında _(Handler)_ değerlendirilmeli ve oluşacak cevaplar aynı yoldan geriye döndürülmelidir. Öyleyse başlayalım.

```shell
# Önce projeyi oluşturalım
cargo new crayz_server
cd crayz_server
# Genel tipleri ayrı bir modül içinde toplayalım
touch src/lib.rs
```

__DEVAM EDECEK__