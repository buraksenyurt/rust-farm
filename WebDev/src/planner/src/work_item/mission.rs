use super::model::completed::Completed;
use super::model::doing::Doing;
use super::model::ready::Ready;
use std::fmt::{Display, Formatter};

/// Görevin kendisini duruma göre tutan enum sabiti.
#[derive(Debug, PartialEq)]
pub enum Mission {
    Ready(Ready),
    Doing(Doing),
    Completed(Completed),
}

impl Display for Mission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready(item) => write!(f, "{}", item.header),
            Self::Doing(item) => write!(f, "{}", item.header),
            Self::Completed(item) => write!(f, "{}", item.header),
        }
    }
}
