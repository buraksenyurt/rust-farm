use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
use crate::Size;

/// Hali hazırda yürütülmekte olan bir işi ifade eder.
#[derive(Debug, PartialEq)]
pub struct Doing {
    pub header: Base,
}

impl Doing {
    pub fn new(input_title: &str, input_size: Size) -> Self {
        Doing {
            header: Base::new(input_title, "Doing", input_size),
        }
    }
}

impl Create for Doing {}
impl Edit for Doing {}
impl Delete for Doing {}
impl Get for Doing {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_doing_works() {
        let job = Doing::new("Odayı temizle.", Size::Short);
        assert_eq!(job.header.status, "Doing");
    }
}
