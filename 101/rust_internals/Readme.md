# Rust Internals

Normal varsayılan olarak bir rust projesini oluşturduğumuzda çalıştırılabilir binary'nin boyutu epeyce büyük olur. 

Sistem : Ubuntu 22.04
Uygulama : termianl projesi
İçerik :

```rust
fn main() {
    println!("Hello, world!");
}
```

İçinde sadece println makrosu kullanan bu uygulama için release aldığımızda oluşan binary'nin boyutu yaklaşık olarak 4 Mb civarındadır. Sistemde tam olarak 4_298_864 Kb.

```shell
# release almak için
cargo build -r

# dosya detayına bakmak için
ls -l target/release/rust_internals
```



