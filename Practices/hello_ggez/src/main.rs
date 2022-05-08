use ggez::conf::Conf;
use ggez::event::{run, EventHandler};
use ggez::{timer, Context, ContextBuilder, GameError};
use std::time::Duration;

/*
   Her oyunun bir döngüsü vardır(Game Loop olarak ifade edilir)
   Oyun döngüsünde klavye, mouse, pencere kapatma gibi olaylar, oyuncunun pozisyonu, puanı
   gibi durum verileri, şekil ve imgelerin ekrana çizilmesi gibi işler ele alınır.

   ggez küfesinde bu iş için EventHandler isimli trait kullanılır.
*/

fn main() {
    // State nesnesi
    let state = State::new();

    // Olası konfigurasyon ayarları
    let c = Conf::new();
    // Her oyunun bir Context ve EventLoop nesnesine ihtiyacı olur
    let (ctx, event_loop) = ContextBuilder::new("ggez_first_sample", "burak selim")
        .default_conf(c)
        .build()
        .unwrap();

    // Oyunu Context, EventLoop ve State nesneleri ile başlattığımız kısım
    run(ctx, event_loop, state);
}

// Oyunun o anki durumuna ait tüm veri ve bilgiler bir State nesnesinde tutulur
// Oyuncu pozisyonları, skorlar, oyun sahasındaki diğer varlıklar vb
// Context enesi ile mouse, klavye, zamanlayıcılar, grafik ve ses gibi donanımlara erişebiliriz.
struct State {
    duration: Duration,
}

impl State {
    pub fn new() -> Self {
        Self {
            duration: Duration::new(0, 0),
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.duration = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        println!("Duration ={}nano saniye", self.duration.as_nanos());
        Ok(())
    }
}
