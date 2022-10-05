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
    
# Projeyi scaffold yardımıyla oluşturmak için
cargo create-tauri-app

# Örnek için
# Project Name -> boaring-todos
# Package Manager -> npm
# UI Template -> vanilla
```

---

**cargo create-tauri-app** komutunun çalışması için sistemde ilgili template'in yüklenmiş olması gerekebilir. Bunun içinde **cargo install create-tauri-app** komutundan yararlanılabilir.

---

![../images/hello_tauri_01.png](../images/hello_tauri_01.png)

```shell
cd boaring-todos
npm install
```
