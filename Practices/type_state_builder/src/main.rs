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
