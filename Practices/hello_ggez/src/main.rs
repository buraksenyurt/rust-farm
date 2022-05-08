use ggez::event::{Axis, Button, ErrorOrigin, EventHandler, GamepadId, KeyMods, MouseButton};
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, GameError};

/*
   Her oyunun bir döngüsü vardır(Game Loop olarak ifade edilir)
   Oyun döngüsünde klavye, mouse, pencere kapatma gibi olaylar, oyuncunun pozisyonu, puanı
   gibi durum verileri, şekil ve imgelerin ekrana çizilmesi gibi işler ele alınır.

   ggez küfesinde bu iş için EventHandler isimli trait kullanılır.
*/

fn main() {
    let state = State {};
}

// Oyunun o anki durumuna ait tüm veri ve bilgiler bir State nesnesinde tutulur
// Oyuncu pozisyonları, skorlar, oyun sahasındaki diğer varlıklar vb
// Context enesi ile mouse, klavye, zamanlayıcılar, grafik ve ses gibi donanımlara erişebiliriz.
struct State {}

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}
