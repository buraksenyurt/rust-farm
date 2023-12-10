# Exercism Problem Çözümleri

- **Easy**
  - [x] 00 Reverse String
  - [x] 01 Gigasecond
  - [ ] 02 Armstrong Numbers
- **Medium**
  - [x] 01 Clock
  - [x] 02 Anagram
  - [x] 03 Space Age
  - [x] 04 Sublist
  - [x] 05 Minesweeper
  - [x] 06 Luhn
- **Hard**
  - [ ] 01 Parallel Letter Frequency
  - [ ] 02 Macros

## Test

Program testlerini koşturmakiçin workspace üstünde aşağıdaki terminal komutları ile ilerlenebilir.

```bash
# sadece clock kütüphanesindeki testleri çalıştırır.
cargo test -p clock

# hatta belli bir test metodunu da aşağıdaki gibi çalıştırabiliriz
cargo test -p clock zero_hour_and_negative_minutes
```