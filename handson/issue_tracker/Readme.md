# Issue Tracker

Bir haftasonu pratiği :D Örnekte bir HTTP Web API geliştirmeye çalışıyorum. Kod iyileştirme çalıştırmaları ile ilgili issue açmak, kapatmak veya listelemek için kullanmayı planlıyorum. Amaçlarımdan birisi minimum seviyede harici küfe (crate) kullanmak.

## Çalışma Zamanı

Örneği çalıştırdıktan sonra Postman, curl gibi araçlarla örnek HTTP talepleri gönderilebilir.

```bash
cargo run

# Hello demek için
http://localhost:8086/hello

# Açık tüm talepleri görmek için
http://localhost:8086/issues
```

İlk etapta sistemde zaten var olan dummy issue verisini denedim. Normalde JSON serileştirme için serde crate'ten yararlanılabilir tabii ama amacım saf rust kodları kullanarak örneği tamamlayabilmek. Yani built-in gelen modül ve işlevsellikler dışına çıkmadan bunu yapabilmek istiyorum.

![../images/issue_tracker_01.png](../images/issue_tracker_01.png)

## Yapmak İstediklerim

- [ ] HTTP Post ile server tarafına Issue kaydı alabilmeliyim.
- [ ] Issue için ters serileştirme işlevleri gerekiyor.
- [ ] Server tarafı asenkron çalışacak hale getirilebilir.
- [ ] Issue'lar ilk etapta sıkıştırılmış bir dosya da saklanabilir _(DB kullanırsam DB ile konuşturma noktasında saf Rust işlevleri ile nasıl ilerleyeceğim. Bu önemli bir soru işaret :D )_

