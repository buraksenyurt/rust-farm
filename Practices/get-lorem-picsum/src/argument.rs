use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Many(List),
    Single(u32),
    Random,
}

#[derive(Debug, PartialEq)]
pub struct List {
    pub page_number: u8,
    pub count: u8,
}

impl List {
    pub fn new(page_number: u8, count: u8) -> Self {
        Self { page_number, count }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Invalid,
}
impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "many" => Ok(Command::Many(List::new(1, 10))),
            "single" => Ok(Command::Single(1)),
            "random" => Ok(Command::Random),
            _ => Err(CommandError::Invalid),
        }
    }
}
