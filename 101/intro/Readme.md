# Rust ile Programlama

Sunum için kullanılan örnek kodları içerir.

## 01 - Dakika Bir Gol Lifetimes

Aşağıdaki kod ile başlanır.

```rust
fn main() {
    let wilson = find_player(23);
    println!("{}", wilson.nick_name);
}

fn find_player(id: i32) -> Player {
    Player {
        id,
        nick_name: "Don Du Dragon Wilsın",
    }
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}
```

**Missing Lifetime Specifier** hatası vurgulanır ve düzeltilir.

```rust
fn main() {
    let wilson = find_player(23);
    println!("{}", wilson.nick_name);
}

fn find_player<'a>(id: i32) -> Player<'a> {
    Player {
        id,
        nick_name: "Don Du Dragon Wilsın",
    }
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}
```

## 02 - Constructor Görevi Gören new Eklenir

Player nesnesinin kolay örneklenmesi için fonksiyon implementasyonuna geçilir. Önce aşağıdaki gibi program kodu değiştirilir.

```rust
fn main() {
    let wilson = Player::new(23,"Can Kilod Van Dam");
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}

impl Player {
    pub fn new(id: i32, nick_name: &str) -> Self {
        Self { id, nick_name }
    }
}
```

**implicit elided lifetime not allowed here** hatası gösterilir ve kod düzeltilerek ilerlenir.

```rust
fn main() {
    let wilson = Player::new(23,"Can Kilod Van Dam");
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str) -> Self {
        Self { id, nick_name }
    }
}
```

## 03 - Serüvene Enum Veri Yapısı Katılır

Oyuncunun seviyesini tutan bir enum sabiti eklenir. Bu enum sabitinde Score şeklinde bir veri tutulması da gösterilir.

```rust
fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

## 04 - Enum Var da Pattern Matching Olmaz mı?

Enum ile tutulan Level'ın durumuna göre karar verilmek istendiğinde pattern matching ile ilerlenebilir. Kod aşağıdaki hale getirilir.

```rust
fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            10..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}/{}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level.win, wilson.level.lose, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

Burada özellikle println! fonksiyonunda level üstünden score bilgilerine erişilemediği görülür ve no field `win` on type `Level` hatası belirtilir. Çözüm için Display trait'lerinin uygulanması sağlanır.

Kod bu durumda aşağıdaki hale getirilir.

```rust
use std::fmt::{Display, Formatter};

fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            10..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

Bu durumda çok daha güzel çalışma zamanı hataları ile karşılaşılır. Borrow Checker devrededir.

```text
error[E0382]: borrow of partially moved value: `wilson.level`
  --> src/main.rs:24:27
   |
10 |         Level::Beginner(s) => match s.win {
   |                         - value partially moved here
...
24 |         wilson.nick_name, wilson.level, revenue
   |                           ^^^^^^^^^^^^ value borrowed here after partial move
```

Score nesnesinin kısmi olarak ödünç alınması nedeniyle referansının başkası tarafından aynı anda ödünç alınamadığını düşünebiliriz. Çözüm için derleyicide önerildiği gibi Score veri yapısına Clone ve Copy trait'lerini uygulayabiliriz.

_Bu arada gözden kaçan bazı durumlar için uyarılara da bakmakta yarar var. Cargo Clippy' de denenebilir_

```rust
use std::fmt::{Display, Formatter};

fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

## 05 - İşin İçerisine Birden Fazla Oyuncu Girdiğinde Vector Kullanılabilir

Bir Vector tanımlayıp wilson buraya eklenir. Immutable olma hali vurgulanır ama daha da önemlisi burada **Use of moved value** probleminin ortaya çıkmasıdır.

```rust
fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(wilson);

    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}
```

Yukarıdaki kod için derleyici şöyle bir hata mesajı verir.

```text
error[E0382]: borrow of moved value: `wilson.level`
  --> src/main.rs:28:27
   |
6  |     let wilson = Player::new(
   |         ------ move occurs because `wilson` has type `Player<'_>`, which does not implement the `Copy` trait
...
11 |     players.push(wilson);
   |                  ------ value moved here
...
28 |         wilson.nick_name, wilson.level, revenue
   |                           ^^^^^^^^^^^^ value borrowed here after move
   |
```

Çözüm noktasında wilson isimli oyuncuyu vektöre referans olarak eklemek kabul edilebilir.

```rust
fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(&wilson);

    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}
```

## 06 N Sayıda Oyuncu Gelir ve Toplu Ödül Hesaplaması için Döngüye Girilir

Birden fazla oyuncunun sisteme eklenmesi ve bunların ödül hesaplamaları için uygulama kodu aşağıdaki gibi değiştirilir.

```rust
fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(&wilson);
    let cesika = Player::new(32, "Jesica Abla", Level::Elit);
    players.push(&cesika);
    let con = Player::new(13, "Con Wik", Level::Beginner(Score { win: 10, lose: 4 }));
    players.push(&con);

    for p in players {
        let revenue = calculate_revenue(p);

        println!(
            "{}({}) isimli oyuncunun ödülü {} coin",
            p.nick_name, p.level, revenue
        );
    }
}

fn calculate_revenue(player: &Player) -> i32 {
    let revenue = match player.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };
    revenue
}
```

Bu örnekte for döngüsü yerine fonksiyonel bir yaklaşım da güdülebilir.

```rust
fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(&wilson);
    let cesika = Player::new(32, "Jesica Abla", Level::Elit);
    players.push(&cesika);
    let con = Player::new(13, "Con Wik", Level::Beginner(Score { win: 10, lose: 4 }));
    players.push(&con);

    players.iter().for_each(|p| {
        let revenue = calculate_revenue(p);
        println!(
            "{}({}) isimli oyuncunun ödülü {} coin",
            p.nick_name, p.level, revenue
        );
    });
}
```

Bir iyileştirme çalışması olarak calculate_revenue fonksiyonuna player referansı göndermek yerine level değeri de yollanabilir.

## 07 Başımıza Dert Açalım -  cannot assign to `p.revenue`, which is behind a `&` reference

Oyuncuları taşıyan vektörümüz esasında oyuncu nesnelerine ait referansları taşıyor. Şimdi level bilgisine göre hesaplanan revenue değerini tutmak için Player veri yapısına revenue eklediğimiz düşünelim.

```rust
use std::fmt::{Display, Formatter};

fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(&wilson);
    let cesika = Player::new(32, "Jesica Abla", Level::Elit);
    players.push(&cesika);
    let con = Player::new(13, "Con Wik", Level::Beginner(Score { win: 10, lose: 4 }));
    players.push(&con);

    players.iter_mut().for_each(|p| {
        p.revenue = calculate_revenue(&p.level);
        println!(
            "{}({}) isimli oyuncunun ödülü {} coin",
            p.nick_name, p.level, p.revenue
        );
    });
}

fn calculate_revenue(level: &Level) -> i32 {
    let revenue = match level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };
    revenue
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
    pub revenue: i32,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
            revenue: 0,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

Bu durumda ** cannot assign to `p.revenue`, which is behind a `&` reference** şeklinde bir hata alırız.

```text
error[E0594]: cannot assign to `p.revenue`, which is behind a `&` reference
  --> src/main.rs:18:9
   |
18 |         p.revenue = calculate_revenue(&p.level);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign
```

Vektörümüzdeki oyuncu nesnelerini tutarken referansları ile tutmuştuk. Dolayısıyla for döngüsüne iter_mut ile nesneleri değiştirilebilir halde taşısak da, Rust derleyicisi referans arkasında kalan bir değişkenin değerinin değiştirilmesine müsaade etmez. _(Bunun sebebini öğrencilerle tartışalım)_ Ancak vektöredeki tutulma biçimlerini değiştirirsek sorunu ortadan kaldırabiliriz. Yani aşağıdaki gibi.

```rust
fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(wilson);
    let cesika = Player::new(32, "Jesica Abla", Level::Elit);
    players.push(cesika);
    let con = Player::new(13, "Con Wik", Level::Beginner(Score { win: 10, lose: 4 }));
    players.push(con);

    players.iter_mut().for_each(|p| {
        p.revenue = calculate_revenue(&p.level);
        println!(
            "{}({}) isimli oyuncunun ödülü {} coin",
            p.nick_name, p.level, p.revenue
        );
    });
}
```

## Notlar

- println! makrosunun yardım kutucuğunu mutlaka gösterelim. lock notuna değinelim. Harici linke gidip kaynak kodunu da açalım.
- 05nci pratikte ikinci bir oyuncuyu players vektörüne eklerken neden referans olarak (& ile) eklenmesini istediğini açıklayalım.
- Revenue değerinin hesaplamasını yapan kod parçasını main'den çıkarıp harici bir fonksiyona alabileceğimiz gibi, Player veri yapısına uygulanmış bir fonksiyonda da konuşlandırabiliriz. Bunu öğrencilerle bir tartışmaya açalım derim.

