# Lorem Picsum'dan Fotoğraf İndirme

Popüler bir lorem ipsum türü olan [bu siteden](https://picsum.photos/) basit REST sorguları ile Unsplash kaynaklı fotoğraflar çekmek mümkün. Bir uygulamada rastgele fotoğraflara ihtiyacımız olduğu hallerde kullanımı ideal. Örnek uygulamada buraya basit REST sorgusu atıp fotoğrafları local bilgisayar ortamına JPG olarak indiren bir terminal uygulaması söz konusu.

Örnek Rest sorgularını şöyle ifade edebiliriz;

| URL                                            | Açıklama                                                                                                                              |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| https://picsum.photos/v2/list?page=2&limit=100 | Sayfalama tekniği ile 2nci sayfadan 100 fotoğraf getirir.                                                                             |
| https://picsum.photos/id/12/info               | Belli bir ID değerine sahip fotoğrafı getirir. Örneğin 12 nolu fotoğraf gibi.                                                         |
| https://picsum.photos/seed/picsum/640/480      | Tam anlamıyla rastgele bir fotoğraf getirir. Bu örnekte 640X480 boyutlarında.                                                         |
| https://picsum.photos/640/480?grayscale        | Yine rastgele bir fotoğraf getirir ama bunu siyah beyaz formata çevirerek yapar. Örnekte 640X480 boyutlarında bir fotoğraf söz konusu |

## Çalışma Zamanı

Örnek terminalden aldığı argümanlara göre talepleri ilgili REST çağrılarına çevirecek ve buna göre birden fazla ya da tek bir fotoğrafı indirecektir.

```bash
# Örnek tamamlanınca eklenecek
```