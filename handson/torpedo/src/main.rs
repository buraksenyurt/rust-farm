use crate::message::Message;
use crate::state_machine::StateMachine;
use anyhow::Context;

mod body;
mod message;
mod payload;
mod state_machine;

fn main() -> anyhow::Result<()> {

    //println!("Node etkin.\nMesajları işlemek için beklemedeyim...");

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout = std::io::stdout().lock();

    let mut sm = StateMachine { id: 0 };
    for input in inputs {
        let input = input.context("Mealstrom'dan gelen girdi Deserialize edilemedi")?;
        //println!("Message {:?}", input);
        sm.step(input, &mut stdout)
            .context("State machine içinde hata")?;
    }
    Ok(())
}
