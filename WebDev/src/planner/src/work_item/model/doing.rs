use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

/// Hali hazırda yürütülmekte olan bir işi ifade eder.
#[derive(Debug, PartialEq)]
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

impl Create for Doing {}
impl Edit for Doing {}
impl Delete for Doing {}
impl Get for Doing {}

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
