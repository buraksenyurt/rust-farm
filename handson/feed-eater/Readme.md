# Feed Eater - Pardon Reader :P

Örnek uygulamada GitLab'dan Michalel Fridrich'in ["Learn Advanced Rust Programming with a Little Help From AI"](https://about.gitlab.com/blog/2023/10/12/learn-advanced-rust-programming-with-a-little-help-from-ai-code-suggestions/) isimli yazısından esinlenerek bir RSS Okuyucu yazmayı öğreniyorum. Yazının aksine bir AI asistanı kullanmadan ilerlemeye karar verdim. Genel amaç terminalden güncel bazı haberleri çekebilmek. Hatta belki basit parametreler ile ilk 5 haberi göster gibi ilerlenebilir de.

Web talepleri için reqwest, feed parsing işleri için feed-rs, environment dosyasından okuma için dotenv isimli crate'ler kullanılmakta.

```bash
cargo add reqwest -F "blocking"
cargo add feed-rs
cargo add dotenv
```