# Sıralama Seçimleri için Strategy Tasarım Kalıbını Kullanmak

Tasarım kalıplarının Rust tarafındaki uygulanmasında trait' ler büyük öneme sahip. Bu örnekte de onlardan faydalanıp örnek bir vakada strategy design pattern' in nasıl uygulanabileceğini incelemeye çalışıyorum. Kobay olarak farklı sıralama algoritmalarının asıl uygulayıcı tarafa strategy deseni üzerinden verilmesi vakasını ele alacağım.

Örnekte yer alan SortStrategy trait'i sıralama fonksiyonunu bildirir. Asıl sıralayıcılar bu trait'i implemente ederler. Context nesnesi SortingMaster isimli structure'dır. new ile oluşturulurken hangi sıralama strateji ile çalışacağı bilgisini alır. change fonksiyonu ile istenirse belirlenen strateji değiştirilebilir. sort operasyonu ise oluşturulurken seçilen sıralama algoritmasını uygulayan nesne üzerinden sıralama işini icra eder. 

Örnekte amaç sıralama algoritmalarının nasıl yazıldığını öğrenmekten ziyade _(ki internette her yerde bulunabilirler ve bende öyle yaptım hatta :D )_ nesne davranış biçimleri ile ilgili olan strategy tasarım kalıbınının rust tarafında nasıl uygulanacağını anlamaktır.

## Çalışma Zamanı Çıktısı

![../images/strategical_sorts_runtime.png](../images/strategical_sorts_runtime.png)

## Çalışmada Kullanılan Enstrümanlar / Kavramlar

- Starteji tasarım kalıbının kullanılması
- Kendi trait modelimizin geliştirilmesi
- Birim testler
- Box pointer ile dynamic dispatch uygulanması

