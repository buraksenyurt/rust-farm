//use std::str::FromStr;
use clap::{Parser, Subcommand};

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

// impl FromStr for Command {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match str {
//             "ping" => Ok(Command::Ping),
//             "get" => Ok(Command::Get),
//             "set" => Ok(Command::Set),
//             _ => Ok(Command::Unknown),
//         }
//     }
// }
