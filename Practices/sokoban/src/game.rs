use crate::prelude::*;

// Oyun nesnesi
pub struct Game {
    pub world: World,
}

// Oyun motoruna ait ana döngü. Şu an için iki davranış uygulanıyor.
// Güncelleme ve çizim
impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // Oyunun güncelleme anlarında girdi sistemini çalıştırıyoruz
        {
            let mut input_system = InputSystem {};
            input_system.run_now(&self.world);
        }
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        info!("{:?} tuşuna basıldı.", keycode);
        let mut input_events = self.world.write_resource::<InputEvents>();
        input_events.pressed_keys.push(keycode);
    }
}
