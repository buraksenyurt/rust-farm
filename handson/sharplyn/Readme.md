# Sharplyn (C# Kod Analizi)

Bu uygulamada amacım C# kodlarını tarayıp Abstract Syntax Tree modellerini Rust tarafında ele alabilmek. Buradan çıkışla C# ile yazılmış proje kodlarını analiz etme ve belki de değiştirme işlerini nasıl yapabileceğimi bulmaya çalışacağım. Roslyn yerine Rust ile fiziki dosya taraması, AST ile kodun semantik analizinin yapılması ve bazı sonuçlar çıkartılıp, düzenlemeler yapılması hedeflerim arasında. AST yapısını kolayca çıkarmak için yardımcı bir crate kullanmayı tercih edebilirim. [syn](https://crates.io/crates/syn) isimli Crate Rust kodları için oldukça başarılı ama ilk denemelerde C# kod taramalarında sorun çıkardı. Anlayamadığı token'lar var, namespace gibi... Bu çok normal. Alternatif başka crate'ler var araştırabildiğim kadarı ile ama ilk etapta string içeriklerini kendim parse ederek token'ları çıkarmayı ve oradan AST modeline geçmeyi deneyeceğim.

Çok basit adımlarla başlamak gerekiyor. Mesela ilk etapta aşağıdaki c# kodunu alıp, Rust tarafındaki struct'lara dönüştürüp bir bağlam kurmaya çalışmak iyi bir yaklaşım olabilir.

```csharp
using System;
using System.Collections.Generic;

public namespace GameBusiness {
    public class State {
    }

    public class Actor {
    }

    public class Event {
    }
}
```