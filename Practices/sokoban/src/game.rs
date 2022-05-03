use crate::prelude::*;

// Oyun nesnesi
pub struct Game {
    pub world: World,
}

// Oyun motoruna ait ana döngü. Şu an için iki davranış uygulanıyor.
// Güncelleme ve çizim
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    // oyunun ana döngüsünde her güncelleme sonrası nesnelerin çizilmesi işlemi uygulanacak
    fn draw(&mut self, context: &mut Context) -> Result<(), GameError> {
        // bu çizim işlemini RenderingSystem üstleniyor.
        {
            let mut rendering_system = RenderingSystem { context };
            rendering_system.run_now(&self.world);
        }
        Ok(())
    }
}
