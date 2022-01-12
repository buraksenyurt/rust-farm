# Ortaya Karışık Pratik Rust Örnekleri

## Dokümantasyon

 ```shell
cargo new doc_sample

# Normalde doküman üretimi için aşağıdaki komut kullanılır
cargo doc

# Ancak bağımlı kütüphanelerin dokümantasyonunu dahil etmek istemezsek
cargo doc --no-deps 

# Hatta geliştirme sırasında şu daha şık olur.
cargo doc --no-deps --open
```

### Notlar

//! ile başlayan yorumlar inner doc olarak anılır. /// Kullandığımızda,

![./images/doc_sample_1.png](./images/doc_sample_1.png)

//! şeklinde değişiklik yapıldığında.

![./images/doc_sample_2.png](./images/doc_sample_2.png)

Kodun idiomatic olması için önerileri clippy verir. _cargo clippy_ 