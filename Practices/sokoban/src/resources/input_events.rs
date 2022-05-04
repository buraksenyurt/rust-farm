use crate::prelude::*;

// Oyunda bazı bilgiler entity, component ve system nesneleri için ortak olabilir.
// Mesela bir tuşa basılması olayı.
#[derive(Default)]
pub struct InputEvents {
    pub pressed_keys: Vec<KeyCode>,
}
