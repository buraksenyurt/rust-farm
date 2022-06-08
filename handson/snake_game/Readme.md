# Snake Game

Rust tarafında kod geliştirdikçe wasm paketini çıkmak gerekir.

```shell
wasm-pack build --target web
```

Web sunucusunu başlatmak içinse www klasöründe aşağıdaki komut çalıştırılır.

```shell
npm run dev
```

## Notlar

drawGameWorld fonksiyonunda yatay ve dikey eksen çizgileri çizidirilmekte. Buradaki döngüleri aşağıdaki grafikle açıklayabiliriz.

x eksenine göre dikey çizgilerin çizdirilmesi.

![../images/snake_game_01.png](../images/snake_game_01.png)

Yılan nesnesinin herhangibir t anındaki veri yapısına ait görüntü aşağıdaki gibi ifade edilebilir.

![../images/snake_game_02.png](../images/snake_game_02.png)

Başlangıçta yılanın ilk karesini çizen fonksiyonun görsel anlatımı aşağıdaki gibi ifade edilebilir.

![../images/snake_game_03.png](../images/snake_game_03.png)

Hücre sağa doğru hareket ettiğinde satır sonuna gelince yine aynı satırın 0ncı hücresinden başlamasının nasıl sağlandığının matematiği.

![../images/snake_game_04.png](../images/snake_game_04.png)

Yön tuşları için eklenen olay dinleyicisi iş başındayken tarayıcı loglarından bir görüntü.

![../images/snake_game_05.png](../images/snake_game_05.png)