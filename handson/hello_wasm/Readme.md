# HTML İçinden WASM Komutlarını İşletmek

İlk olarak [https://webassembly.github.io/wabt/demo/wat2wasm/](https://webassembly.github.io/wabt/demo/wat2wasm/) adresine gidilir ve örneklerden esinlenilerek basit bir *WAT(Web Assembly Text)* kodu oluşturulur. Örneğin,

```wasm
(module
  (func (export "calc") (param f32) (result f32)
    f32.const 3.14    
    local.get 0    
    f32.mul
    local.get 0
    f32.mul
))
```

gibi. Burada calc isimli bir fonksiyon tanımlanmıştır. 32 bit float tam sayı alıp yine aynı türde sonuç döndürür. İlk satırında 3.14 pi değeri sabit olarak tanımlanmış ardından get 0 ile fonksiyon parametresi çekilmiş sonrasında mul çağrısı ile de çarpma işlemi gerçekleştirilmiştir. Ardından fonksiyon parametresi tekrar hesaba katılmış ve pi * r * r işlemi gerçekleştirilmiştir. 