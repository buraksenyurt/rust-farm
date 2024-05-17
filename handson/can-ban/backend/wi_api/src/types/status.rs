use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Status {
    Todo = 1,
    Inprogress = 2,
    Completed = 3,
}

impl TryFrom<u8> for Status {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Self::Todo as u8 => Ok(Self::Todo),
            v if v == Self::Inprogress as u8 => Ok(Self::Inprogress),
            v if v == Self::Completed as u8 => Ok(Self::Completed),
            _ => Err(()),
        }
    }
}
