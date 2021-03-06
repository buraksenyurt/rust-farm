use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
use crate::Size;

/// Tamamlanmış bir işi ifade eder.
#[derive(Debug, PartialEq)]
pub struct Completed {
    pub header: Base,
}

impl Completed {
    pub fn new(input_title: &str, input_size: Size) -> Self {
        Completed {
            header: Base::new(input_title, "Completed", input_size),
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
        let job = Completed::new("Odayı temizle.", Size::Short);
        assert_eq!(job.header.status, "Completed");
    }
}
