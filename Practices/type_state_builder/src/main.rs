#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("World build error: {0}")]
    World(&'static str),
}
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    /*
       State 00 : Başlangıç

       World oluşutmak için WorldBuilder veri modelini aşağıdaki
       gibi kullandığımızı düşünelim. Burada name alanına değer atanmamakta.
       Bu build fonksiyonundaki kontrol sebebiyle çalışma zamanında,
       hata üretilmesine neden olacaktır.

       Error: World("empty name")

       Amacımız name ve background alanlarının mutlaka girilmesi gerektiğini,
       derleme zamanında görebilmek. Type State Builder desenin burada devreye giriyor.
    */
    // let mordor = WorldBuilder::new()
    //     //.name("Mordor")
    //     .background("middle_earth_big.jpg")
    //     .window(1024., 768.)
    //     .build()?;
    // println!("{mordor:#?}");

    /*
       Type State Uygulanması

       DİKKAT! Uygulamanın amacını görmek için,
       name veya background fonksiyon çağrılarını kapatıp

       cargo build

       deneyin.

    */
    let gondor = GameBuilder::new()
        .name("Gondor")
        .background("lord_of_the_rings_map_big.jpg")
        .window(1280., 900.)
        .build()?;
    println!("{gondor:#?}");

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

/*
   State 01 : Type State Pattern Uyarlaması

   name ve background özellikleri için derleme zamanında hata üretimi yaptırmak istiyoruz.
   Type State deseni kullanacağız. Bunun için önce bu iki özelliğin olası durumlarını tanımlarız.

   Üstteki örnekle karışmaması için burada Window yerine WindowSize,
   World yerine Game ve WorldBuilder yerine GameBuilder kullanılmıştır.
*/

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
