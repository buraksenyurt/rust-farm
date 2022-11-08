Bu örnekte MongoDb veritabanı kullanılıyor. Linux tabanlı sistemimde onu çalıştırmak için aşağıdaki terminal komutunu kullandım.

```shell
sudo docker run -d -p 27017-27019:27017-27019 --name gondor mongo
sudo docker container ps -a

# container shell'in geçmek için
sudo docker exec -it gondor bash

# mongodb komutlarını çalıştırmak içinse
mongosh
# veritabanlarını gösterir
show dbs
# bir veritabanı oluşturalım
use AdventureWorks
# Temsili bir kayıt eklemek için
db.category.insertOne({title:"Book"})
# eklenen içeriği görmek için
db.category.find().pretty()

#mongosh içinde ekranı temizlemek için
cls
# mongosh 'dan çıkmak için
exit
# container terminalinden çıkmak için
exit
```

Örnek bir veri ekleme işlemi için aşağıdaki JSON içeriği kullanılabilir.

```json
{
    "title":"Kahve fincan",
    "stock_level":100,
    "price":12.90,
    "category":"Mutfak Malzemeleri"
}
```

Bunu uygulamayı çalıştırdıktan sonra _(cargo run)_ http://localhost:8000/product adresinden deneyebiliriz. Ben Postman'den yararlandım.

![../images/mongo_rocket_01.png](../images/mongo_rocket_01.png)

Dikkat edileceği üzere eklenen ürün bilgisini görüntülemek için aşağıdaki mongosh komutu kullanılmıştır.

```text
db.Product.find().pretty()
```

Bir dokümanı aramak içinse HTTP Get çağrısı yapılabilir. Parametre ilgili ürünün object id bilgisidir.

![../images/mongo_rocket_02.png](../images/mongo_rocket_02.png)