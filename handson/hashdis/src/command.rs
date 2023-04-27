use clap::{Parser, Subcommand};
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct Keyword {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Unknown,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ping" => Ok(Command::Ping),
            "get" => Ok(Command::Get {
                key: "".to_string(),
            }),
            "set" => Ok(Command::Set {
                key: "".to_string(),
                value: "".to_string(),
            }),
            _ => Ok(Command::Unknown),
        }
    }
}
