use std::fmt::{Display, Formatter};

/// Görev durum bilgisini tutan enum sabiti
pub enum Status {
    Ready,
    Doing,
    Completed,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ready => write!(f, "Ready"),
            Status::Doing => write!(f, "Doing"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}
