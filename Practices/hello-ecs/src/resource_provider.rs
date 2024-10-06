use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ResourceProvider {
    store: HashMap<TypeId, Box<dyn Any>>,
}
impl ResourceProvider {
    pub fn add(&mut self, data: impl Any) {
        self.store.insert(data.type_id(), Box::new(data));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.store.get(&type_id) {
            data.downcast_ref::<T>()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_delta_time_into_resource_store_test() {
        let resource_provider = setup_resource_provider();
        let store = resource_provider.store.get(&TypeId::of::<DeltaTime>()).unwrap();
        let delta_time_resource = store.downcast_ref::<DeltaTime>().unwrap();
        assert_eq!(delta_time_resource.0, 0.5);
    }

    #[test]
    fn get_delta_time_from_resource_store_test() {
        let resource_provider = setup_resource_provider();
        if let Some(delta_time) = resource_provider.get::<DeltaTime>() {
            assert_eq!(delta_time.0, 0.5);
        }
    }

    fn setup_resource_provider() -> ResourceProvider {
        let mut resource_provider = ResourceProvider::default();
        let delta_time = DeltaTime(0.5);
        resource_provider.add(delta_time);
        return resource_provider;
    }

    struct DeltaTime(pub f32);
}
