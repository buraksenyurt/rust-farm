# Boring Todo App

Backend tarafında Rust'ın kullanıldığı bir Todo uygulaması esasında. Program PostgreSQL veritabanını kullanmakta. Benim gibi sistemine PostgreSQL veritabanını yüklemek istemeyenler pekala docker imajlarından yararlanabilirler. Bunun için aşağıdaki komutlar yeterli olacaktır.

```shell
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=scoth_tiger" --name pg postgres:14.5
```

Ben sistemimde daha önceden yüklemiş olduğum postgresql imajını kullanıyorum. Örneğin ilk kısmında kod tarafındaki bazı script dosyalarını kullanarak veri tabanı, tablo ve örnek verilerin oluşturulması işlemleri söz konusu. Oluşan görüntü ilk etapta tatmin edici.

![assets/screenshot_01.png](assets/screenshot_1.png)