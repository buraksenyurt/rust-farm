# Sharplyn (C# Kod Analizi)

Bu uygulamada amacım C# kodlarını tarayıp Abstract Syntax Tree modellerini Rust tarafında ele alabilmek. Buradan çıkışla C# ile yazılmış proje kodlarını analiz etme ve belki de değiştirme işlerini nasıl yapabileceğimi bulmaya çalışacağım. Roslyn yerine Rust ile fiziki dosya taraması, AST ile kodun semantik analizinin yapılması ve bazı sonuçlar çıkartılıp, düzenlemeler yapılması hedeflerim arasında. AST'ti çıkarmak için yardımcı bir crate kullanmayı tercih edeceğim. [syn](https://crates.io/crates/syn) isimli Crate'ten yararlanabilirim.

