#[derive(Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    List,
    Invalid(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.trim().splitn(3, ' ');
        let cmd = parts.next().unwrap_or("").to_uppercase();

        match cmd.as_str() {
            "SET" => {
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.next().unwrap_or("").to_string();
                Command::Set { key, value }
            }
            "GET" => {
                let key = parts.next().unwrap_or("").to_string();
                Command::Get { key }
            }
            "REMOVE" => {
                let key = parts.next().unwrap_or("").to_string();
                Command::Remove { key }
            }
            "LIST" => Command::List,
            _ => Command::Invalid(cmd),
        }
    }
}
