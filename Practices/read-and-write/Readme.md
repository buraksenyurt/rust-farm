# Temel IO İşleri

from_file ve to_file isimli örnekler BufReader ve BufWriter kullanarak dosyadan veri okuma veya yazma işlemleri için kullanılırlar. BufReader ve BufWriter, küçük boyutlu ve sık olarak gerçekleşen okuma ile yazma işlemleri için performanslıdır ancak çok büyük boyutlu verilerde ekstra bir performans kazanımı sağlamaz.

Linux ve Unix sistemlerde bir process ve çevre birimleri arasındaki iletişim stream'ler ile sağlanır. Linux ve Unix sistemleri açısından bakıldığında 3 önemli stream vardır. Standart girdi(Standard Input), standart çıktı(Standard Output) ve standart hata (Standard Error). Örneğin bir terminal uygulaması klavye girdilerini veya farklı bir process'den gelen girdiyi okurken Standard Input stream kullanılır. Bir process(uygulama) terminale çıktı verirken ya da başka bir process'e çıktı gönderirken de Standard Output stream kullanılır. Çoğu modern sistemde Standard Error stream'i process'in log dosyasına bağlıdır ve bu içeriğin analiz edilmesinde kullanılır.  

## Çalıştırmak İçin

```bash
# dosya okuma örneği
cargo run --bin from_file

# dosyaya yazma örneği
cargo run --bin to_file

# stdin, stdout, stderr kullanımları örneği
cargo run --bin in_and_out

# iterator kullanımı örneği
cargo run --bin iterator

# dosyaları birleştirerek okuma(chaining) örneği
cargo run --bin chaining
```