use crate::prelude::*;

// Hareket ettirilmemesi gereken entity nesnelerini belirtmek için bu bileşen kullanılıyor.
// Mesela kilitli sandık, yer ve duvar gibi nesneler hareket edemez.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable {}
