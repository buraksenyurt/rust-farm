# Rust ile Kafka Kullanımı

Bu örnekte herhangibir servis kaynağından yakaladığı JSON içeriklerini Kafka'ya yayınlayan ve bu yayınlayan bilgileri dinleyip okuyan iki Rust programı kullanılmakta. Local makinede Kafka'yı ayağa kaldırmak için docker-compose'dan yararlanılıyor.

İlk denemeler için [şu adresteki ücretsiz](https://saurav.tech/NewsAPI/top-headlines/category/technology/in.json) veri çıktısını kullanabiliriz. Tabii farklı haber kaynaklarına da uygulayabiliriz. 

```bash
sudo docker-compose up

# Kafka tarafında topic açmak için
sudo docker exec broker \
  kafka-topics --bootstrap-server broker:9092 \
  --create --topic technews
  
# topic'leri görmek içinde aşağıdaki komutu kullanabiliriz
sudo docker exec broker \
  kafka-topics --bootstrap-server broker:9092 \
  --list
  
# topic bilgisini silmek istersek
sudo docker exec broker \
  kafka-topics --bootstrap-server broker:9092 \
  --delete --topic technews
```

İlk önce **publisher** isimli uygulamayı çalıştırmak lazım. Sonrasında da **subscriber** isimli örneği.