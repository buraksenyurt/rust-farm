# Tauri ile Masaüstü Uygulama Geliştirmek

Rust tarafında cross-platform masaüstü uygulamalar geliştirmek için kullanılan paketlerden birisi [tauri](https://tauri.app/). Teori oldukça basit. HTML, CSS, Javascript gibi önyüz tarafını inşa etmekte kullanılabilecek enstrümanları arka plan kodlamasının yapıldığı Rust ile uyumlu halde çalıştırmak. Ben denememi kendi Ubuntu 22.04 sistemimde yapmaktayım. Deneme sonrasında önreği Windows ve MacOS işletim sistemlerinde çalıştırmak da istiyorum.

Sistemde Rust ve nodejs yüklü ancak Tauri için bazı ön gereksinimler var. Bu gereksinimlerin neler olduğunu öğrenmek ve sistemime nasıl yükleyeceğimiz görmek için resmi dokümantasyondan yararlandım.

```shell
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

İlgili gereksinimler tamamlandıysa örneğe başlayabiliriz.

## Projenin İnşaası

```shell
# önce User Interface klasörünü oluşturalım
# içerisine basit bir HTML sayfası ekleyelim
mkdir frontend
cd frontend/index.html

# Şimdi backend tarafını inşa edeceğiz
# Burada tauri-cli aracından yararlanacağız.
# Bu nedenle onu sisteme yüklemek lazım.(Yoksa tabii) 
cargo install tauri-cli

# Projenin oluşturulması
cargo tauri init

# Ben sorulara şöyle cevap verdim
# app name -> BoaringTodos
# title -> My Boaring Todos
# HTML/CSS/JS lokasyonu -> ../frontend
# url of your dev server -> ../frontend
# frontend dev command -> npm run dev
# frontend build command -> npm run build
```

![../images/hello_tauri_01.png](../images/hello_tauri_01.png)