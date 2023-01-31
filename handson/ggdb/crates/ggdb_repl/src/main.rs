use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    println!("GGDB(CiCi DB)\nSonlandırmak için CTRL+D\n");
    let mut reader = Editor::<()>::new()?;
    if reader.load_history(HISTORY_FILE).is_err() {
        println!("Arşivde bilgi bulunamadı");
    }
    loop {
        let readline = reader.readline(">> ");
        match readline {
            Ok(line) => {
                reader.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    reader.save_history(HISTORY_FILE)
}
