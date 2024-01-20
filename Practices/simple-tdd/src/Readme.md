# Rust ile TDD Pratiği

TDD _(Test Driven Development)_ yüksek kalite standartlarında, test edilmiş, güvenilirliği yüksek kodlar üretmek için önemli metodolojilerden birisidir. Programlama dillerinin tamamında bu metodolojiyi uygulamak mümkündür. Bu metodolojiye Red Green Blue ya da Red Green Refactor dendiği de olur. Teorisi basittir. Önce sisteme eklenecen fonksiyonellik için fail duruma düşüren birim test yazılır. Ardından Green safhasına geçilir ve fonksiyon testi başarılı çalışacak biçimde tamamlanır. Mavi renkle ifade edilen Refactor safhasında ise fonksiyonun temiz ve etkili olması için gerekli iyileştirmeler yapılır. Bu konuyu uygulamalı olarak gösterirken kullanılabilecek en basit örneklerden birisi de faktöryel fonksiyonunun TDD ile yazılmasıdır.

## Red (Fail State)

Aşağıdaki kodlama ile işe başlanır.

```rust
pub fn factorial(_number: u64) -> u64 {
    unimplemented!()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_of_0_test() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1_test() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_4_test() {
        assert_eq!(factorial(4), 24);
    }
}
```

Burada faktöryel hesabı yapan fonksiyon için 3 test yazılmıştır. 0 ve 1 için faktöryel değer 1 dönerken, 4 için 24 dönmesi beklenir. Bu aşam esasında fonksiyonun parametre yapısının, dönüş değerinin belirlendiği ve kabul kriterlerinin inşa edildiği safhadır diye düşünebiliriz. Çok doğal olarak cargo test çalıştırıldığında unimplemented makrosunun kullanılması sebebiyle birim testlerini üçü de fail olacaktır.

![../images/simple_tdd_red.png](../images/simple_tdd_red.png)

## Green (Pass State)



## Blue (Refactor State)