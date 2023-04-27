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

pub enum ServerCommand {
    Ping,
    Get,
    Set,
    Unknown,
}

impl FromStr for ServerCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ping" => Ok(ServerCommand::Ping),
            "get" => Ok(ServerCommand::Get),
            "set" => Ok(ServerCommand::Set),
            _ => Ok(ServerCommand::Unknown),
        }
    }
}
