use super::base::Base;

#[derive(Debug, PartialEq)]
pub struct Ready {
    pub header: Base,
}

impl Ready {
    pub fn new(input_title: &str, business_value: u16) -> Self {
        Ready {
            header: Base::new(input_title, business_value, "Ready"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_ready_works() {
        let job = Ready::new("Odayı temizle", 5);
        assert_eq!(job.header.status, "Ready");
        assert_eq!(job.header.value, 5);
    }
}
