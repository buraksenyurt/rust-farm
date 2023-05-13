# Temel IO İşleri

from_file ve to_file isimli örnekler BufReader ve BufWriter kullanarak dosyadan veri okuma veya yazma işlemleri için kullanılırlar. BufReader ve BufWriter, küçük boyutlu ve sık olarak gerçekleşen okuma ile yazma işlemleri için performanslıdır ancak çok büyük boyutlu verilerde ekstra bir performans kazanımı sağlamaz.

## Çalıştırmak İçin

```bash
cargo run --bin from_file

cargo run --bin to_file
```