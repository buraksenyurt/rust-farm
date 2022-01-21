# Nucleotide Count Alıştırması

Bu alıştırmada bir DNA diziliminin, DNA dizilimi olup olmadığını anlamak için bir fonksiyon geliştirilmesi isteniyor. İşin içinde  adenin (A), guanin (G), sitozin (C) ve timin (T) dizilimi var. Fonksiyonun bu harf dizilimlerine bakması ve toplam sayılarını döndürmesi gerekiyor. Buna uymayan durumlarda ise hata vermesi isteniyor.

Örneğin;

"GATTACA" -> 'A': 3, 'C': 1, 'G': 1, 'T': 2
"INVALID" -> error

gibi...

```shell
cargo new nucleot --lib
cargo clippy
cargo test
```