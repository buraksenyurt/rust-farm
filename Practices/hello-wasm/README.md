# Bir Web Assembly Denemesi

Web tarafını ayağa kaldırmak için node ve npm'e ihtiyaç olacaktır. Moon *(Benim ubuntu sistemi)* üstünde bunlar yoktu.

```shell
sudo apt update
sudo apt install nodejs

# Kurulum başarılı olduysa bir versiyona bakılır
node -v

# npm yüklemek için
sudo apt install npm

# tabii birde webpack'e ihtiyaç var
sudo npm install --save-dev webpack

# işleri kolaylaştırmak için hazır bir proje şablonu var
# aşağıdaki komut ile bunu indirmek lazım.
cargo install cargo-generate

# sonrasında git üstündeki proje şablonu ile yeni bir proje oluşturulur.
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

![../images/hello_wasm_01.png](../images/hello_wasm_01.png)