# GameStore

Bu örnekte Rocket ve SeaORM küfeleri kullanılarak REST tabanlı bir web api hizmeti örneklenmeye çalışılıyor. Uygulamada bağımsız oyunlar ve bunları geliştiren programcı bilgileri kullanılmakta. Veriyi mysql veritabanında tutmayı planlıyoruz. ORM aracı olarak sea-orm küfesini, REST tarafı servis kabiliyetleri ve çalışma zamanı için de Rocket küfesini kullanacağız.

## Proje Bağımlılıkları

Rust tarafında kullanılan küfelerin yüklenmesi. _(Doğrudan Cargo.toml dosyası içerisine de yazılabilirler)_

```bash
# rest routing sistemi için
cargo add rocket -F json
# orm tarafı için
cargo add sea-orm -F sqlx-mysql,runtime-async-std-native-tls,macros
# orm migration işlemleri için
cargo add sea-orm-migrator
# JWT Token için
cargo add jsonwebtoken
# Kullanıcı şifrelerini encrypt etmek için
cargo add bcrypt
# Environment değerlerini okumak için
cargo add dotenvy

# migrator işleri için sea-orm-cli kullanılmakta
cargo install sea-orm-cli

# migrator klasörünü ve ilk sürümü oluşturmak için aşağıdaki komut çalıştırılabilir
sea migrate init -d ./src/migrator

# ilk etapta üretilen migration planında gerekli hazırlıklardan sonra
# diğer tablolarla ilgili migration planları da ayrıca hazırlanabilir
# geliştiriciler ile ilgili olan için aşağıdaki yapı kullanılabilir
sea migrate generate -d ./src/migrator create_developer_table

# game tablosu için de aşağıdaki gibi
sea migrate generate -d ./src/migrator create_game_table
```

## Veritabanı Tarafı

Bu örnekte mysql veritabanı kullanmakta. İşi kolaylaştırmak için docker-compose şablonu kullanıyorum. Ayağa kaldırmak için aşağıdaki komutu çalıştırmak yeterli.

```bash
sudo docker-compose.yml up
```

yml içeriğinde Adminer isimli bir veritabanı yönetim aracı da bulunuyor. http://localhost:8085/ adresinden MYSQL veritabanına bağlanarak işlerimizi kolaylaştırabiliriz. Söz gelimi bu örnek için gamestore isimli bir veritabanı gerekli. Bunu oluşturmak için ilgili arabirimi pekala kullanabiliriz.

![../images/game_store_01.png](../images/game_store_01.png)

![../images/game_store_02.png](../images/game_store_02.png)

![../images/game_store_03.png](../images/game_store_03.png)

Program ilk çalıştırıldığında migration planı başarılı bir şekilde yürütülürse Adminer arabiriminde aşağıdakine benzer bir görüntü ile karşılaşmamız gerekir.

![../images/game_store_04.png](../images/game_store_04.png)

## Entity Nesnelerinin Üretilmesi

Migration planlarının çalıştırılması sadece MySQL tarafındaki tabloların oluşturulmasını sağlar. Kod tarafında tablolara karşılık gelecek entity nesneleri için de seq-orm CLI aracından yararlanılabilir. Bu örnek için aşağıdaki şekilde ilerlenilmiştir.

```bash
sea generate entity -o src/entity -u mysql://root:tiger@localhost:3306/gamestore
```

![../images/game_store_05.png](../images/game_store_05.png)

## API Operasyonları

Uygulamadaki API hizmetlerine ait şablonları şöyle belirleyebiliriz.

- **User Tarafı**
  - Sign In, (/auth/sign-in), HTTP Post
  - Sing Up, (/auth/sign-up), HTTP Post
- **Developer Tarafı**
  - List, (/developers), HTTP Get
  - Detail, (/developers/{id}), HTTP Get
  - Create, (/developers), HTTP Post
  - Update, (/developers/{id}), HTTP Put
  - Delete, (/developers/{id}), HTTP Delete
- **Game Tarafı**
  - List, (/games), HTTP Get
  - Detail, (/games/{id}), HTTP Get
  - Create, (/games), HTTP Post
  - Update, (/games/{id}), HTTP Put
  - Delete, (/delete/{id}), HTTP Delete
  
## Testler

Yeni bir kullanıcı oluşturulması. Bunun için postman tarafından aşağıdaki talebin çalıştırılması yeterlidir.

**Adres : localhost:8000/auth/sign-up**

```json
{
    "email": "noone@nowhere.com",
    "password": "doe@1234!",
    "firstname": "John",
    "surname": "Doe"
}
```

![../images/game_store_06.png](../images/game_store_06.png)

Zaten kayıtlı olan olan bir email adresi ile kayıt olmaya çalışıldığında ise...

![../images/game_store_07.png](../images/game_store_07.png)

Kayıtlı bir kullanıcı başarılı şekilde login olduğunda bir JWT token elde etmesi gerekir.

**Adres : localhost:8000/auth/sign-in**

```json
{
    "email": "noone@nowhere.com",
    "password": "doe@1234!"
}
```

![../images/game_store_08.png](../images/game_store_08.png)

Servisin developers ve games isimli endpoint'lerini kullanabilmek için geçerli bir JWT token'a sahip olmak ve yine bu token'la ilintili geçerli bir Identity bilgisine sahip olmak gerekiyor. Öncelikle geçerli bir Identity alma hizmetine bakalım. Bunun için aşağıdaki request örneği kullanılabilir.

**Adres: localhost:8000/auth/identity**

Sign-in sonrası ele edilen token bilgisini Header'da ekleyerek göndermek gerekir. Token geçerli ise güncel kullanıcıya ait id bilgisinin alınması beklenir. Örnekte token, baseurl gibi bilgiler Postman collection'a ait variable'lardan gelmektedir.

![../images/game_store_09.png](../images/game_store_09.png)

Geçersiz bir token kullanılır veya token yollanmazsa da HTTP 401 Unauthorized hatası alınır.

![../images/game_store_10.png](../images/game_store_10.png)

Yeni bir geliştirici aşağıdaki gibi eklenebilir.

**Adres: localhost:8000/developers**

method : HTTP Post
```json
{
    "fullname": "Monkey Boy",
    "about": "A famous indie game developer",
    "level": 50
}
```

![../images/game_store_11.png](../images/game_store_11.png)

Yeni bir oyun içeriği de aşağıdaki gibi eklenebilir.

**Adres: localhost:8000/games**

method : HTTP Post
```json
{
  "developer_id": 1,
  "title": "Flying Dutch Man",
  "year": "1998",
  "summary": "a funny sailing game"
}
```

![../images/game_store_12.png](../images/game_store_12.png)

Belli bir id değerine sahip oyun bilgisini çekmek için aşağıdaki gibi ilerlenebilir.

**Adres: localhost:8000/games/1**

![../images/game_store_14.png](../images/game_store_14.png)

Benzer şekilde belli bir id değerine sahip programcı bilgisini çekmek içinse aşağıdaki gibi ilerlenebilir.

**Adres: localhost:8000/developers/1**

![../images/game_store_13.png](../images/game_store_13.png)

Görüldüğü üzere MySQL tarafı için çalıştırılan SQL sorguları log olarak terminal ekranına da düşmektedir.

Programcı veya oyun bilgilerini güncellemek istediğimizde standart olarak HTTP Update operasyonunu kullanırız. Örnek çalışmadaki 1 numaralı Developer bilgisini güncellemek istediğimizde aşağıdaki gibi ilerleyebiliriz.

**Adres: localhost:8000/developers/1**

```json
{
    "fullname": "Monkey The Ram",
    "about": "A famous indie game developer from Çeska",
    "level": 55
}
```

![../images/game_store_15.png](../images/game_store_15.png)

Bir oyun bilgisinin güncellenmesi de aşağıdaki görselde olduğu gibidir.

![../images/game_store_16.png](../images/game_store_16.png)