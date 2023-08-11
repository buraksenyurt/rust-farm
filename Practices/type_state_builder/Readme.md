# Type State Builder Pattern

Örnekte World isimli bir veri modelini Consumer Builder Pattern'e göre örneklemenin yolu gösterilmektedir. Uygulamanın başlangıçtaki hali aşağıdaki kod parçasında olduğu gibidir.

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("World build error: {0}")]
    World(&'static str),
}
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let mordor = WorldBuilder::new()
        .background("middle_earth_big.jpg")
        .window(1024., 768.)
        .build()?;
    println!("{mordor:#?}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Window {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct World {
    name: String,
    background: String,
    window: Option<Window>,
}

#[derive(Default, Clone)]
struct WorldBuilder {
    name: Option<String>,
    background: Option<String>,
    window: Option<Window>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        WorldBuilder::default()
    }

    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    fn background(mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self
    }

    fn window(mut self, width: f32, height: f32) -> Self {
        self.window = Some(Window { width, height });
        self
    }

    fn build(self) -> Result<World> {
        let Some(n) = self.name.as_ref() else {
            return Err(Error::World("empty name"));
        };
        let Some(bg) = self.background.as_ref() else {
            return Err(Error::World("invalid background"));
        };
        let window = self.window.unwrap_or_else(|| Window {
            width: 640.,
            height: 480.,
        });

        Ok(World {
            name: n.to_string(),
            background: bg.to_string(),
            window: Some(window),
        })
    }
}
```

Burada vurgulanmak istenen nokta name ve background alanlarına değer atanmadığında ne olacağıdır. build fonksiyonundaki kontroller sebebiyle çalışma zamanında bir hata fırlatılır ve program sonlanır. Type State Builder Pattern nimetlerinden yararlanarak name veya background alanlarına değer atanmadığı durumda derleme zamanında hata ürettirilmesi sağlanabilir. Buna göre Game ve GameBuilder için söz konusu durum aşağıdaki kod parçasında ele alınmaktadır.

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("World build error: {0}")]
    World(&'static str),
}
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    // Type State Uygulanması
    let gondor = GameBuilder::new()
        // .name("Gondor")
        //.background("lord_of_the_rings_map_big.jpg")
        .window(1280., 900.)
        .build()?;
    println!("{gondor:#?}");

    Ok(())
}

#[derive(Default, Clone)]
struct NoName;
#[derive(Default, Clone)]
struct Name(String);

#[derive(Default, Clone)]
struct NoBackground;
#[derive(Default, Clone)]
struct Background(String);

#[derive(Debug, Clone)]
struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Game {
    name: String,
    background: String,
    window: Option<Window>,
}

#[derive(Default, Clone)]
struct GameBuilder<N, B> {
    name: N,
    background: B,
    window: Option<Window>,
}

impl GameBuilder<NoName, NoBackground> {
    fn new() -> Self {
        GameBuilder::default()
    }
}

impl GameBuilder<Name, Background> {
    fn build(self) -> Result<Game> {
        let window = self.window.unwrap_or_else(|| Window {
            width: 640.,
            height: 480.,
        });

        Ok(Game {
            name: self.name.0,
            background: self.background.0,
            window: Some(window),
        })
    }
}

impl<N, B> GameBuilder<N, B> {
    fn name(mut self, name: impl Into<String>) -> GameBuilder<Name, B> {
        GameBuilder {
            name: Name(name.into()),
            background: self.background,
            window: self.window,
        }
    }

    fn background(mut self, background: impl Into<String>) -> GameBuilder<N, Background> {
        GameBuilder {
            name: self.name,
            background: Background(background.into()),
            window: self.window,
        }
    }

    fn window(mut self, width: f32, height: f32) -> Self {
        self.window = Some(Window { width, height });
        self
    }
}
```

Örnekte gondor nesnesi örneklenirken name ve background değerleri atanmamıştır. Bu durumda derleme zamanında aşağıdaki gibi bir hata mesajı ile karşılaşılır.

```text
error[E0599]: no method named `build` found for struct `GameBuilder<NoName, NoBackground>` in the current scope
   --> src/main.rs:34:10
    |
30  |       let gondor = GameBuilder::new()
    |  __________________-
31  | |         // .name("Gondor")
32  | |         //.background("lord_of_the_rings_map_big.jpg")
33  | |         .window(1280., 900.)
34  | |         .build()?;
    | |         -^^^^^ method not found in `GameBuilder<NoName, NoBackground>`
    | |_________|
    | 
...
134 |   struct GameBuilder<N, B> {
    |   ------------------------ method `build` not found for this struct
    |
    = note: the method was found for
            - `GameBuilder<Name, Background>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `type_state_builder` (bin "type_state_builder") due to previous error
```

Burada hata mesajındaki 'method not found in `GameBuilder<NoName, NoBackground>`' a dikkat edelim. Bu mesaj, hem name hem de background beklendiğine dair bir ipucudur. Keza sadece name alanına değer verip background'a değer vermezsek derleme zamanı hata mesajı buna uygun ipucu verecek şekilde değişir.

```text
error[E0599]: no method named `build` found for struct `GameBuilder<Name, NoBackground>` in the current scope
   --> src/main.rs:34:10
    |
30  |       let gondor = GameBuilder::new()
    |  __________________-
31  | |         .name("Gondor")
32  | |         //.background("lord_of_the_rings_map_big.jpg")
33  | |         .window(1280., 900.)
34  | |         .build()?;
    | |         -^^^^^ method not found in `GameBuilder<Name, NoBackground>`
    | |_________|
    | 
...
134 |   struct GameBuilder<N, B> {
    |   ------------------------ method `build` not found for this struct
    |
    = note: the method was found for
            - `GameBuilder<Name, Background>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `type_state_builder` (bin "type_state_builder") due to previous error
```

Dikkat edileceği üzere method not found in `GameBuilder<Name, NoBackground>` şeklinde bir bildirim var. Yani Name alanı verilmiş ama background yok gibi düşünebiliriz. Tam tersi durum da geçerlidir. background alanına değer verip name alanını atlarsak derleme zamanı hata mesajı buna göre evrilir.

```text
error[E0599]: no method named `build` found for struct `GameBuilder<NoName, Background>` in the current scope
   --> src/main.rs:34:10
    |
30  |       let gondor = GameBuilder::new()
    |  __________________-
31  | |         //.name("Gondor")
32  | |         .background("lord_of_the_rings_map_big.jpg")
33  | |         .window(1280., 900.)
34  | |         .build()?;
    | |         -^^^^^ method not found in `GameBuilder<NoName, Background>`
    | |_________|
    | 
...
134 |   struct GameBuilder<N, B> {
    |   ------------------------ method `build` not found for this struct
    |
    = note: the method was found for
            - `GameBuilder<Name, Background>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `type_state_builder` (bin "type_state_builder") due to previous error
```

