use crate::command::{Command, Keyword, ServerCommand};
use crate::utility::Utility;
use async_trait::async_trait;
use bytes::BytesMut;
use clap::Parser;
use log::{error, info, warn};
use std::io::Error;
use std::str::{from_utf8, FromStr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ServerProcessor;

#[async_trait]
pub trait Processor {
    async fn run(&self, stream: &mut TcpStream) -> Result<(), Error>;
}

#[async_trait]
impl Processor for ServerProcessor {
    async fn run(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let mut buffer = BytesMut::with_capacity(512);
        stream.read_buf(&mut buffer).await?;
        let incoming_commands = Utility::convert_to_vec(&mut buffer);
        let first_command = ServerCommand::from_str(incoming_commands[0].as_str());

        if let Ok(c) = first_command {
            info!("İstemciden gelen içerik {:?}", incoming_commands);
            match c {
                ServerCommand::Ping => {
                    println!("İstemci ping attı...");
                    stream.write_all(b"pong").await?;
                }
                ServerCommand::Get => {}
                ServerCommand::Set => {}
                ServerCommand::Unknown => {}
            }
        }

        Ok(())
    }
}

pub struct ClientProcessor;

#[async_trait]
impl Processor for ClientProcessor {
    async fn run(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let arguments = Keyword::parse();
        match arguments.command {
            Command::Ping => match stream.write_all(b"ping").await {
                Ok(_) => {
                    let mut buffer = BytesMut::with_capacity(512);
                    stream.read_buf(&mut buffer).await?;
                    if let Ok(r) = from_utf8(&buffer) {
                        println!("{}", r);
                    }
                    Ok(())
                }
                Err(e) => {
                    error!("Veri gönderilemedi. {}", e);
                    Err(e)
                }
            },
            Command::Set { key, value } => {
                stream.write_all(b"set").await?;
                stream.write_all(b" ").await?;
                stream.write_all(key.as_bytes()).await?;
                stream.write_all(b" ").await?;
                stream.write_all(value.as_bytes()).await?;
                Ok(())
            }
            Command::Get { key } => {
                stream.write_all(b"get").await?;
                stream.write_all(b" ").await?;
                stream.write_all(key.as_bytes()).await?;
                Ok(())
            }
            _ => {
                warn!("Komut anlaşılamadı");
                Ok(())
            }
        }
    }
}
