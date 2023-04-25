use crate::message::Message;
use crate::state_machine::StateMachine;
use anyhow::Context;

mod body;
mod message;
mod payload;
mod state_machine;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let stdout = std::io::stdout().lock();
    let mut output = serde_json::Serializer::new(stdout);

    let mut sm = StateMachine { id: 0 };
    for input in inputs {
        let input = input.context("Mealstrom'dan gelen girdi Deserialize edilemedi")?;
        sm.step(input, &mut output)
            .context("State machine içinde hata")?;
    }
    Ok(())
}
