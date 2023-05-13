use std::io::{stderr, stdin, stdout, Read, Write};

fn main() -> std::io::Result<()> {
    /*
    Standard Input/Output Stream kullanımı (Echo örneği)
     */

    let mut buffer = String::new();
    // Terminalden bir metin girdiğimizde stdin bunu buffer isimli String'e okur
    let _ = stdin().read_line(&mut buffer)?;
    // Girdiğimiz içerik terminale çıktı olarak basılır
    stdout().write(&mut buffer.as_bytes())?;

    /*
    Lock kullanımı.
    Input ve Output stream nesneleri process'e ait veriyi paylaşırlar.
    Dolayısıyla bazen verinin başkası tarafından kullanımını engellemek için ilgilie
    stream nesnelerine kilit(lock) koymak gerekebilir.
     */
    let mut buffer = [8; 512];

    let input_s = stdin();
    let mut locked_input = input_s.lock();
    locked_input.read(&mut buffer)?;

    let output_s = stdout();
    let mut locked_output = output_s.lock();
    locked_output.write(&mut buffer)?;

    let buffer = b"Eeeee...Houston...We have a problem!\n";
    let error_s = stderr();
    let mut locked_error = error_s.lock();
    locked_error.write(buffer)?;

    Ok(())
}
