# Boring Todo App

Backend tarafında Rust'ın kullanıldığı bir Todo uygulaması esasında. Program PostgreSQL veritabanını kullanmakta. Benim gibi sistemine PostgreSQL veritabanını yüklemek istemeyenler pekala docker imajlarından yararlanabilirler. Bunun için aşağıdaki komutlar yeterli olacaktır.

```shell
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=scoth_tiger" --name pg postgres:14.5
```