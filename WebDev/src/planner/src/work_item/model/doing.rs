use super::base::Base;

pub struct Doing {
    pub header: Base,
}

impl Doing {
    pub fn new(input_title: &str, business_value: u16) -> Self {
        Doing {
            header: Base::new(input_title, business_value, "Doing"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_doing_works() {
        let job = Doing::new("Odayı temizle.", 5);
        assert_eq!(job.header.status, "Doing");
        assert_eq!(job.header.value, 5);
    }
}
