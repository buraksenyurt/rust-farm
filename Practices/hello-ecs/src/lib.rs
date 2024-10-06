mod resource_provider;

use crate::resource_provider::ResourceProvider;
use std::any::Any;

#[derive(Default, Debug)]
pub struct World {
    resource_provider: ResourceProvider,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_resource(&mut self, resource: impl Any) {
        self.resource_provider.add(resource);
    }

    pub fn get_resource<T: Any>(&mut self) -> Option<&T> {
        self.resource_provider.get::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resource_provider.get_mut::<T>()
    }
}
