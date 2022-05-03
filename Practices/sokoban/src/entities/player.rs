use crate::prelude::*;

// oyuncu entity nesnesini oluşturmak için kullanılan fonksiyon
// Dikkat edilmesi gereken nokta create_entity sonrası çağırılan fonksiyonlar.
// Oyuncu nesnesini Position ve Renderable bileşenleri ile birlikte oluşturmaktayız
// ki Player bileşeni de buna dahil.
// Bileşenler dikkat edileceği üzere sadece veri içeren yapılar. İşlevleri yok.
pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            asset_path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}
