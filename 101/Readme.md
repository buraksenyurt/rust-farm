# Rust Programlama Dili Temelleri _(Başlangıç Seviyesi)_

Rust dilinin temellerini ve sahip olduğu genel enstrümanların nasıl kullanıldığını öğrenmek amacıyla oluşturulmuş bir dokümandır.

## Örnekler

Gerekli açıklamalar kod satırları arasında yer almaktadır.

### Module00: Hello World

```shell
# hello_world isimli yeni bir proje oluşturulur.
cargo new hello_world

# çalıştırmak için
cargo run
```

![images/mod00.png](images/mod00.png)

### Module01: newton _(Test yazmak, Doküman Oluşturmak)_

```shell
# Yeni fonksiyonlar taşıyan kütüphaneler oluştururken 
cargo new newton --lib

# Test koşturmak için
cargo test

# Yardım dokümanını oluşturmak için
cargo doc

# sonrasında doc/newton/index.html'e bakılabilir
```

![images/mod01_2.png](images/mod01_2.png)

/// ifadeleri ile oluşturulan yardımcı bilgiler html dosyası üzerinden görüntülenebilir. Tüm Rust çevresindeki standart doküman formatı budur ve markdown tabanlıdır.

![images/mod01_1.png](images/mod01_1.png)

Yardım dokümanları fonksiyon gibi öğelerin kullanımında geliştiriciye önemli ipuçları verir.

![images/mod01_3.png](images/mod01_3.png)

### Module02: scalar_types _(Temel Veri Türleri)_

__Scalar Data Types__

    Integer:

    Size    Signed      Unsigned

    8 bit   i8          u8
    16 bit  i16         u16
    32 bit  i32         u32 (tamsayılar için varsayılan türdür)
    64 bit  i64         u64
    128 bit i128        u128
    arch    isize       usize (mikro işlemci mimarisine göre)

    Float:

    32 bit  f32
    64 bit  f64 (noktalı sayılar için varsayılan türdür)

    Boolean:

    true
    false

    Char:

```shell
# Projeyi oluşturmak için
cargo new scalar_types

# Programı çalıştırmak için
cargo run
```

Çalışma zamanından basit bir görüntü.

![images/mod02_1.png](images/mod02_1.png)

## Module03: Basit Fonksiyonlar _(some_functions)_

Temel fonksiyon kullanımlarına bir bakalım.

```shell
# Kütüphane olarak oluşturalım.
cargo new simple_functions --lib

# test etmek için
cargo test
```

Çalışma zamanına ait örnek görüntü.

![images/mod03_1.png](images/mod03_1.png)

Hemen araya reklam olarak faydalı iki _cargo_ komutu alalım.

```shell
# Kod içeriğini RustFmt'te göre formatlar.
# Yani terminalden de format düzenlemeleri yaptırabiliriz.
cargo fmt

# Bu ise kodun idiomatic olmasını sağlar.
# Yani kodun Rust'ın getirdiği özellikler kullanılarak daha düzgün yazılması için önerilerde bulunur.
cargo clippy
```

Yukarıdaki örnekte yer alan birkaç fonksiyonu aşağıdaki hale getirelim.

```rust

```