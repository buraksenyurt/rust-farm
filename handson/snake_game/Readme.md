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