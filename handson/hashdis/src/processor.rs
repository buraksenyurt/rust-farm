use crate::command::{Command, Keyword, ServerCommand};
use crate::db::Db;
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
    async fn run(&self, stream: &mut TcpStream, db: Option<&mut Db>) -> Result<(), Error>;
}

#[async_trait]
impl Processor for ServerProcessor {
    async fn run(&self, stream: &mut TcpStream, db: Option<&mut Db>) -> Result<(), Error> {
        let mut buffer = BytesMut::with_capacity(1024);
        stream.read_buf(&mut buffer).await?;
        info!("Buffer büyüklüğü {}", buffer.len());
        let incoming_commands = Utility::convert_to_vec(&mut buffer);
        let first_command = ServerCommand::from_str(incoming_commands[0].as_str());

        if let Ok(c) = first_command {
            info!("İstemciden gelen içerik {:?}", incoming_commands);
            match c {
                ServerCommand::Ping => {
                    println!("İstemci ping attı...");
                    stream.write_all(b"pong").await?;
                }
                ServerCommand::Get => {
                    let key = incoming_commands[1].clone();
                    let read_result = db.unwrap().read(key);
                    match read_result {
                        Ok(r) => {
                            stream.write_all(r).await?;
                        }
                        Err(e) => {
                            error!("{}", e);
                            stream.write_all(b"key not found").await?;
                        }
                    }
                }
                ServerCommand::Set => {
                    let key = incoming_commands[1].clone();
                    let value = incoming_commands[2].clone();
                    let write_result = db.unwrap().write(key, value);
                    match write_result {
                        Ok(r) => {
                            info!("insert işlemi sonucu {}", r);
                            stream.write_all(r.as_bytes()).await?;
                        }
                        Err(e) => {
                            error!("{}", e)
                        }
                    }
                }
                ServerCommand::Unknown => {
                    error!("İstemciden gelen komutlar anlaşılamadı");
                }
            }
        }

        Ok(())
    }
}

pub struct ClientProcessor;

#[async_trait]
impl Processor for ClientProcessor {
    async fn run(&self, stream: &mut TcpStream, _db: Option<&mut Db>) -> Result<(), Error> {
        let arguments = Keyword::parse();
        match arguments.command {
            Command::Ping => match stream.write_all(b"ping").await {
                Ok(_) => {
                    Self::read_server_response(stream).await?;
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
                stream.flush().await?;
                Self::read_server_response(stream).await?;

                Ok(())
            }
            Command::Get { key } => {
                stream.write_all(b"get").await?;
                stream.write_all(b" ").await?;
                stream.write_all(key.as_bytes()).await?;
                stream.flush().await?;
                Self::read_server_response(stream).await?;

                Ok(())
            }
            _ => {
                warn!("Komut anlaşılamadı");
                Ok(())
            }
        }
    }
}

impl ClientProcessor {
    async fn read_server_response(stream: &mut TcpStream) -> Result<(), Error> {
        let mut buffer = BytesMut::with_capacity(1024);
        stream.read_buf(&mut buffer).await?;
        if let Ok(r) = from_utf8(&buffer) {
            println!("{}", r);
        }
        Ok(())
    }
}
