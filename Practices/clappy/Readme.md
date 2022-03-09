# Komut Satırı Argümanları için clap Küfesini Kullanmak

Sistem programlama söz konusu ise olmazsa olmazlardan birisi programa terminal ekranında gönderilen komutlardır. Docker'ı, redis'i vs düşünün. Terminal'den ne çok ve çeşitte parametreler alabiliyorlar. Bu programların main fonksiyonlarındaki argüman yönetimini mümkün mertebe kolaylaştırmak lazım. Rust'ın popüler küfelerinden _(crate)_ olan clap, builder pattern kabiliyetini de işin içerisine katarak bunu epeyce kolaylaştırmakta. Pratik olarak nasıl kullanıldığını deneyimlemek istedim.

```shell
cargo new clappy
```

İlk iş doğal olarak toml dosyasında gerekli bildirimi yapmak.

```toml

```