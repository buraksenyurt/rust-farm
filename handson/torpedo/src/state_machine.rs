use crate::body::Body;
use crate::message::Message;
use crate::payload::Payload;
use anyhow::Context;
use serde::Serialize;
use std::io::StdoutLock;

pub struct StateMachine {
    pub id: usize,
}

impl StateMachine {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply_message = Message {
                    source: input.destination,
                    destination: input.source,
                    body: Body {
                        body_type: "".to_string(),
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                reply_message
                    .serialize(output)
                    .context("Reply mesaj serileştirilemedi")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}
