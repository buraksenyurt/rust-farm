# Tauri ile Basit Bir Masaüstü Uygulamasının Geliştirilmesi

Çalışmadaki amacım Ubuntu üstünde çalışan basit bir masaüstü uygulaması geliştirmek. Backend tarafında rust kullanılan bu uygulamayı daha sonra bir Windows ve Mac sisteminde çalıştırmayı da denemek istiyorum.

---

**Rust tarafında masaüstü uygulamalarını cross-platform olarak geliştirmek için bir çok crate mevcut. Tauri bunlardan birisi. [Tauri ile ilgili detaylı bilgiye bu adresten ulaşabilirsiniz](https://tauri.app/)** 

---

Örneğin Ubuntu 22.04 üstünde denemekteyim. Sistemde nodejs ve rust ekipmanları yüklü. Tauri projesini oluşturmak için npx aracından yararlanmak mümkün. Projeyi oluştururken aşağıdaki seçimleri yaparak ilerledim.

![../images/hello_tauri_01.png](../images/hello_tauri_01.png)

Önyüz tarafı için Vanilla HTML, CSS ve Javascript kullanılmakta. Backend tarafı ise Rust ile yürüyor.