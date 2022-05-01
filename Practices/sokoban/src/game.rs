use ggez::{event, Context, GameError};

// Oyun nesnesi
pub struct Game {}

// Oyun motoruna ait ana döngü. Şu an için iki davranış uygulanıyor.
// Güncelleme ve çizim
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}
