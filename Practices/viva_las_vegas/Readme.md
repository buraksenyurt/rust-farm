# Lifetimes Mevzusu

Rust'ın özellikle Garbage Collector kullanılan dillerden çok farklı olduğunu bellek yönetimi için getirdiği kurallardan dolayı biliyoruz. Ownership, borowwing gibi hususlar sayesinde güvenli bir bellek ortamını garanti etmek üzerine ihtisas yapmış bir dil. Bunlar pek çok dilde otomatik yönetildiği için Rust'ı öğrenmek zaman alabiliyor. Zor konulardan birisi de Lifetimes mevzusu. Bu konuyu esasında 45 Byte sohbetlerinde dile getirmek istiyorum. Lakin çok basit bir örnek gerekiyor. viva_las_vegas bu amaçla yazıldı.

```shell
# Önce projemizi bir oluşturalım
cargo new viva_las_vegas
cd viva_las_vegas
cargo clippy
cargo run
```

Senaryomuz bir oyuncuyu temsil eden veri modelinin tasarlanması ile başlıyor. Bu haliyle gayet güzel çalışmakta.

![../images/viva_las_vegas_1.png](../images/viva_las_vegas_1.png)

