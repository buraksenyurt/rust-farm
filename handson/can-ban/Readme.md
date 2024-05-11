# Can-Ban Board

Bildiğimiz Kanban board'u Rust, Wasm ve HTML üçlemesinde geliştirmek istesem nasıl yaparım diye yola çıktığım örnektir. Solution içerisinde iki klasör yer alıyor. backend klasörü Rest tabanlı çalışan bir Web API için. Kanban board'daki varlıkların temel CRUD _(Create, Read, Update, Delete)_ operasyonları için kullanılıyor diyebiliriz. Frontend klasöründe önyüz uygulamamız yer alacak. Frontend taraftaki uygulamada basit HTML, Rust ve WASM üçlemesi söz konusu. UI ekranında yapılan işlemler için backend taraftaki servise gidildiğini ifade edebiliriz.

Örneğin çalışma zamanı üç aşağı beş yukarı aşağıdaki ekran görüntülerinde olduğu gibidir.

![runtime 01](../images/can_ban_runtime_01.png)

![runtime 02](../images/can_ban_runtime_02.png)

## Frontend Tarafı

Bu projede wasm kullanılacağı için işleri kolaylaştıracak wasm-pack'e ihtiyacımız var.

```bash
# wasm pack kurulumu için
cargo install wasm-pack

# WASM paketini hazırlamak için
wasm-pack build --target web
```

## Backend Tarafı

Backend servisi tipik bir Rest servisi. Log çıktılarını görebilmek için aşağıdaki gibi hareket edebiliriz.

```bash
RUST_LOG=info cargo run
```

## Sunucu Yayınlama

Frontend taraftaki rust tabanlı wasm uygulamasını host etmek için node.js ve express kullanılabilir

```bash
# root klasördeyken
touch server.js

npm init -y

# Express modülü host işlemlerimizi kolaylaştırır
npm install express

# Frontend taraftaki örneği çalıştırmak için, yine root klasördeyken aşağıdaki komut kullanılabilir
npm start
```

## Çalışma Zamanı

Projeler can-ban isimli workspace altında birleştirilmiştir. Kolayca başlatmak için run.sh isimli terminal script'i kullanılabilir. 

```bash
# run.sh betiğini çalıştırılabilir hale getirmek için
sudo chmod +x run.sh

# Sonrasında çalıştırmak için
./run.sh
```
