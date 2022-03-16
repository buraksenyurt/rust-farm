use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Get,
    Create,
    Delete,
    Edit,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "create" => Ok(Command::Create),
            "get" => Ok(Command::Get),
            "delete" => Ok(Command::Delete),
            "edit" => Ok(Command::Edit),
            _ => Err("Komut bulunamadı".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::Command;
    use std::str::FromStr;

    #[test]
    fn should_parse_valid_commands() {
        assert_eq!(Command::from_str("create").unwrap(), Command::Create);
        assert_eq!(Command::from_str("edit").unwrap(), Command::Edit);
        assert_eq!(Command::from_str("delete").unwrap(), Command::Delete);
        assert_eq!(Command::from_str("get").unwrap(), Command::Get);
    }

    #[test]
    #[should_panic]
    fn should_invalid_commands_panics() {
        assert_eq!(Command::from_str("ping").unwrap(), Command::Create);
    }
}
