use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

/// Tamamlanmış bir işi ifade eder.
#[derive(Debug, PartialEq)]
pub struct Completed {
    pub header: Base,
}

impl Completed {
    pub fn new(input_title: &str, business_value: u16) -> Self {
        Completed {
            header: Base::new(input_title, business_value, "Completed"),
        }
    }
}

impl Get for Completed {}
impl Edit for Completed {}
impl Delete for Completed {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_completed_works() {
        let job = Completed::new("Odayı temizle.", 5);
        assert_eq!(job.header.status, "Completed");
        assert_eq!(job.header.value, 5);
    }
}
