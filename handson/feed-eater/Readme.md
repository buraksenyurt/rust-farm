# Feed Eater - Pardon Reader :P

Örnek uygulamada GitLab'dan Michalel Fridrich'in ["Learn Advanced Rust Programming with a Little Help From AI"](https://about.gitlab.com/blog/2023/10/12/learn-advanced-rust-programming-with-a-little-help-from-ai-code-suggestions/) isimli yazısından esinlenerek bir RSS Okuyucu yazmayı öğreniyorum. Yazının aksine bir AI asistanı kullanmadan ilerlemeye karar verdim. Genel amaç terminalden güncel bazı haberleri çekebilmek. Hatta belki basit parametreler ile ilk 5 haberi göster gibi ilerlenebilir de.

Web talepleri için reqwest, feed parsing işleri için feed-rs, environment dosyasından okuma için dotenv isimli crate'ler kullanılmakta.

```bash
cargo add reqwest -F "blocking"
cargo add feed-rs
cargo add dotenv

# Bazı Test komutları

# Feed listesinden her bir feed için ilk 3 girdinin çekilmesi
cargo run -- top 3

# Tüm feed içeriklerinin alınması
cargo run -- all

# Güncel feedlerin listesi
cargo run -- feeds

# Yeni bir feed eklemek için(örnek)
cargo run -- add TechRadarComputingNews https://www.techradar.com/rss/news/computing

# Feed listesinden seçerek feed silmek için
cargo run -- del
```