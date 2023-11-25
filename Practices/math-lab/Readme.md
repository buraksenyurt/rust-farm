# Virtual Workspace Çalışması

Rust ile yazılan projeleri sanal bir Workspace altında toplayabiliriz. Bu örnekte root Cargo.toml dosyasında buna ilişkin bir takım ayarlar yer almaktadır.

Workspace içerisine yeni bir rust projesi eklemeden önce bunun root Cargo.toml dosyasındaki members kısmında bildirilmesi ve sonrasında cargo new komutu ile oluşturulması tercih edilmelidir.

Workspace içerisinde tanımlı bir crate'i diğer crate'ler içinde kullanabilmek için root Cargo.toml'deki dependencies sekmesinde gerekli bildirimlerin yapılması gerekir. Bunun kolay bir yolu cargo add komutunu kullanmaktır.

Bazı durumlarda Workspace içerisindeki bir crate'in build, test gibi işlerden muaf tutulması istenebilir. Bu durumda root klasördeki Cargo.toml dosyasında yer alan exclude sekmesinde bildirim yapmak gerekir.

```bash
# default-members söz konusu ise
# build komutu sadece orada tanımlı projeleri derler
cargo build
# default-members kullanıldığında tüm projeleri derlemek için
# aşağıdaki komut kullanılabilir.
cargo build --workspace

# bu durum testler için de geçerlidir.
# Yani default-members kullanılırsa tüm workspace testlerini yürütmek için
# aşağıdaki komut kullanılır.
cargo test --workspace

# root klasördeyken aşağıdaki komutu yazarsak binary app uygulamasında
# basic isimli crate kullanılabilir hale gelir 
cargo add basic
# aşağıdaki gibi yazıp iki paketi birden ekleyebiliriz de
cargo add basic algebra
```