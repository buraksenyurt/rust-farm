/// İmge dönüştürmede kullanılan boyut
pub enum Size {
    Small,
    Medium,
    Large,
}

impl From<Size> for u32 {
    fn from(size: Size) -> Self {
        match size {
            Size::Small => 50_u32,
            Size::Medium => 100_u32,
            Size::Large => 200_u32,
        }
    }
}
