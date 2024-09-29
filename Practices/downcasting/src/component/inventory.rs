use std::any::Any;

#[derive(Debug)]
pub struct Gem {
    pub value: u32,
}

#[derive(Debug)]
pub struct FieldGlass {
    pub range: u32,
    pub range_unit: RangeUnit,
}

#[derive(Debug)]
pub struct Chest {
    pub origin: String,
    pub volume: f32,
    pub content: String,
}

#[derive(Debug)]
pub enum RangeUnit {
    Km,
    SeaMile,
}

// Herhangibir tipte nesne topluluğu saklamak için Any trait'inden yararlanılabilir
pub struct Inventory {
    pub objects: Vec<Box<dyn Any + 'static>>,
}

impl Inventory {
    pub fn get<T: Any + 'static>(&self, index: usize) -> Option<&T> {
        if index >= self.objects.len() {
            return None;
        }
        self.objects[index].downcast_ref()
    }

    pub fn get_all<T: Any + 'static>(&self) -> Vec<&T> {
        self.objects
            .iter()
            .filter_map(|object| object.downcast_ref::<T>())
            .collect()
    }
}
