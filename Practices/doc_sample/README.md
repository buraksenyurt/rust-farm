# Kod için yardım dokümanı oluşturmak.

 ```shell
cargo new doc_sample

# Normalde doküman üretimi için aşağıdaki komut kullanılır
cargo doc

# Ancak bağımlı kütüphanelerin dokümantasyonunu dahil etmek istemezsek
cargo doc --no-deps 

# Hatta geliştirme sırasında şu daha şık olur.
cargo doc --no-deps --open

# doküman içine eklenmiş gerçek kod parçaları varsa test edebiliriz
cargo test --doc
```

## Notlar

Yardım dokümanları Markdown formatındadır. Yani dokümanda link verebilir, resim gösterebilir, bullet list, heading vs kullanabiliriz.
//! ile başlayan yorumlar inner doc olarak anılır. /// olarak ullandığımızda aşağıdaki sonuçları elde ederiz.

![../images/doc_sample_1.png](../images/doc_sample_1.png)

//! şeklinde değişiklik yapıldığında.

![../images/doc_sample_2.png](../images/doc_sample_2.png)

Kodun idiomatic olması için önerileri clippy verir. _cargo clippy_ Bunu alışkanlık haline getirmekte yarar var. Kodları idiomatic hale getirmek, daha temiz daha şık ve standartlara uygun çıktı üretmek için önemli bir çalışmadır. Örneğin bu kodda to_string yerine Display trait'ini implemente etmenin daha uygun olacağı söyleniyor.

![../images/doc_sample_3.png](../images/doc_sample_3.png)

Examples kısmında örnek kod parçası kullandığımızda dokümanı test ederek doğruluğundan emin olabiliriz.

![../images/doc_sample_6.png](../images/doc_sample_6.png)

Uyarılardaki gibi eksik bildirimleri eklediğimizdeki durum ise şöyledir.

![../images/doc_sample_9.png](../images/doc_sample_9.png)

Sorunları çözünce de böyle bir çıktı elde ederiz.

![../images/doc_sample_7.png](../images/doc_sample_7.png)

Yardım dokümanı kod yazarken de işimizi kolaylaştırır.

![../images/doc_sample_8.png](../images/doc_sample_8.png)

Sonuç,

![../images/doc_sample_4.png](../images/doc_sample_4.png)

Dokümantasyon oluşturmada nasıl bir yol izleneceğine dair en güzel kaynak Rust'ın var olan yardım dokümanlarıdır.
