use super::model::completed::Completed;
use super::model::doing::Doing;
use super::model::ready::Ready;

/// Görevin kendisini duruma göre tutan enum sabiti.
#[derive(Debug, PartialEq)]
pub enum Mission {
    Ready(Ready),
    Doing(Doing),
    Completed(Completed),
}
