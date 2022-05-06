use crate::prelude::*;

// Hareket ettirilebilir entity nesnelerini belirtmek için bu bileşen kullanılıyor.
// Örneğin oyuncu ve sandıklar hareket ettirilebilir varlıklar.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable {}
