# Concurrency Dünyası

Örnek antrenmandaki amacım rust tarafında thread yönetimi ile ilgili konuları hatırlamak. 

Başlamada önce hatırlatıcı not. Concurrency ve Parallelism farklarını aşağıdaki şekille özetleyebiliriz.

![../images/threading_again_01.png](../images/threading_again_01.png)

- stage_1: İlk örnekte rust'ın standart kütüphanesinin kullandığı 1:1 thread modeli ele alınır. 

## Çalışma Zamanı

```bash
cargo run --bin stage_1
```

![../images/threading_again_02.png](../images/threading_again_02.png)

