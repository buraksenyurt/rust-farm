# BookShop gRPC Service

Rust ile ilgili küfeleri _(crate)_ kullanarak basit bir gRPC servisi nasıl yazılır öğrenmeye çalışıyorum. Bu örnekte 
- gRPC servis çalışma zamanı için tonic,
- asenkron çalışma zamanı için tokio,
- proto dosyasından gerekli rust kodlarını üretme için tonic-build
- protocol buffer desteği sağlaması için de prost 

isimli crate'ler kullanılmakta.

```bash
# küfeleri eklemek için
cargo add tonic prost 
cargo add tokio -F "macros","rt-multi-thread"
cargo add --build tonic-build
```