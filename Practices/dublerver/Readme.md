# Bir Uygulamada İki TcpListener Kullanmak

Sırf meraktan denediğim bir örnek. Uygulama çalıştığında iki TcpListener ayağa kaldırıyor. Böylece iki farklı porttan mesaj dinleyen bir programa sahip olabiliriz. Esasında çok mantıksız bir senaryo sayılmaz. Söz gelimi GET taleplerini farklı bir porttan ele alırken POST,PUT,DELETE gibi talepleri diğer porttan ele alacağımızı düşünebiliriz. CQRS'in HTTP taleplerine uygulanan bir versiyonu gibi düşünülebilir belki de.

```shell
cargo new dublerver

cd dublerver
touch src/lib.rs
# ideomatic'liği kontrol için
cargo clippy

# programı çalıştırmak için
cargo run
```