# JWT Token Kullanımı

Bu örnekte tokio küfesinden de yararlanarak JWT token tabanlı bir REST servisi nasıl inşa edilir öğrenmeye çalışıyorum. Temel olarak kullanıcı kaydetme ve login işlemleri sonrası elde edilen token bilgisi ile yetkiye bazlı dummy endpoint'ler çalıştırılmakta. İlk başta kullanıcı bilgilerini bellekte tuttum ancak sonrasında bunun için Postgresql veritabanını kullandım.

## Örnek Talepler

Diğer yandan talepleri daha kolay işletmek adına [postman](Rust%20Jwt%20Tutorial.postman_collection.json) dosyasını da kullanabilirsiniz.

```shell
#Yeni kullanıcı kayıt etme.
#Admin rolünde bir kullanıcı
curl -X POST 'localhost:5555/register' -H "Content-Type: application/json" -d '{"username": "scoth", "password": "tiger@1234", "role": "admin"}'

# Normal User rolünde bir kullanıcı
curl -X POST 'localhost:5555/register' -H "Content-Type: application/json" -d '{"username": "edison", "password": "edison@1234", "role": "user"}'

#Login olma örnekleri
curl -X POST 'localhost:5555/login' -H "Content-Type: application/json" -d '{"username": "scoth", "password": "tiger@1234"}'

curl -X POST 'localhost:5555/login' -H "Content-Type: application/json" -d '{"username": "edison", "password": "edison@1234"}'

#Aşağıdakileri denemek için tabii önce Login olmak gerekli. Login başarılı olduğunda dönen token bilgisi alınıp
#Bearer sonrasındaki kısma yazılmalı.

#Sadece Admin yetkisi ile girilen stats alanı için
curl -X GET 'localhost:5555/stats' -H 'Authorization: Bearer token_bilgisi_gelir'

#Sadece User yetkisindekilerin erişebildiği categories alanı
curl -X GET 'localhost:5555/categories' -H 'Authorization: Bearer token_bilgisi_gelir'
```

## PostgreSQL Entegrasyonu

Kullanıcı bilgileri örneğin ilk sürümünde basit bir vektör tipinden tutuluyordu. Sonraki sürümde ise bu bilgiler için Postgresql'i kullanmayı denedim. Postgresql için docker-compose.yml dosyası kullanılmakta. Gerekli bazı komutlar şöyle;

```shell
# container'ları ayağa kaldırmak için
sudo docker-compose up

# çalışan containerları görmek, başlatmak, durdurmak veya silmek için
# listele
sudo docker ps -a
# başlat
sudo docker start [container_name]
# durdur
sudo docker stop [container_name]
# container'ı kaldır
sudo docker rm [container_id]
# imajı kaldır
sudo docker rmi [image_id]
```