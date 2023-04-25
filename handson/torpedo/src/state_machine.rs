use crate::body::Body;
use crate::message::Message;
use crate::payload::Payload;
use anyhow::{bail, Context};
use std::io::{StdoutLock, Write};

pub struct StateMachine {
    pub id: usize,
}

impl StateMachine {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply_message = Message {
                    source: input.destination,
                    destination: input.source,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply_message)
                    .context("Reply mesaj serileştirilemedi")?;
                output
                    .write_all(b"\n")
                    .context("yeni satır eklerken hata")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
            Payload::Init { .. } => {
                let reply_message = Message {
                    source: input.destination,
                    destination: input.source,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply_message)
                    .context("Init Reply mesaj serileştirilemedi")?;
                output
                    .write_all(b"\n")
                    .context("yeni satır eklerken hata")?;
                self.id += 1;
            }
            Payload::InitOk => bail!("Init ok"),
        }

        Ok(())
    }
}
