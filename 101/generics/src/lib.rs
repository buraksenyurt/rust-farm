#[cfg(test)]
mod tests {
    use crate::entity::Region;

    #[test]
    fn generic_location_test() {
        let golden_horn = Region::<f32> {
            x: 3.14,
            y: -0.0003,
            z: 10.999,
        };
        assert_eq!(golden_horn.z, 10.999);

        let deep_space = Region::<u8>::new(3, 5, 8);
        assert_eq!(deep_space.x, 3);
    }
}

mod entity {
    // T türüyle çalışan bir struct. x,y,z alanları T türüne bürünür.
    pub struct Region<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Region<T> {
        // generic fonksiyon uygulanması
        pub fn new(x: T, y: T, z: T) -> Self {
            Region { x, y, z }
        }
    }
}
