use std::env;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let address = env::var("KIVI_ADDRESS").unwrap_or_else(|_| "127.0.0.1:5555".to_string());
    let mut stream = TcpStream::connect(&address).await?;
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    println!("kivi-store istemcisi - {} adresine bağlandı", address);
    println!("Komutlar: SET key value | GET key | REMOVE key | LIST (çıkmak için Ctrl+C)\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let line = match stdin.next_line().await? {
            Some(line) if !line.trim().is_empty() => line,
            Some(_) => continue,
            None => break,
        };

        stream.write_all(format!("{}\r\n", line).as_bytes()).await?;

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Bağlantı sunucu tarafından kapatıldı.");
            break;
        }
        print!("{}", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}
