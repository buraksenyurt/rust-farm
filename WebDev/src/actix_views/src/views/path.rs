pub struct Path {
    pub prefix: String,
}

// book klasörü altındaki view'ların hazırlanmasında gerekli olan path tanımı için kullanılan bir fonksiyona sahiptir.
impl Path {
    pub fn define(&self, following_path: String) -> String {
        self.prefix.to_owned() + &following_path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_define_works() {
        let base_path = Path {
            prefix: String::from("/book"),
        };
        let get_path = base_path.define(String::from("/get"));
        assert_eq!(get_path, "/book/get");

        let get_path = base_path.define(String::from("/update"));
        assert_eq!(get_path, "/book/update");
    }
}
