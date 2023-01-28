use std::io;

fn main() {
    let letter_rows=vec![
        " #  ##   ## ##  ### ###  ## # # ###  ## # # #   # # ###  #  ##   #  ##   ## ### # # # # # # # # # # ### ###",
        "# # # # #   # # #   #   #   # #  #    # # # #   ### # # # # # # # # # # #    #  # # # # # # # # # #   #   #",
        "### ##  #   # # ##  ##  # # ###  #    # ##  #   ### # # # # ##  # # ##   #   #  # # # # ###  #   #   #   ##",
        "# # # # #   # # #   #   # # # #  #  # # # # #   # # # # # # #    ## # #   #  #  # # # # ### # #  #  #      ",
        "# # ##   ## ##  ### #    ## # # ###  #  # # ### # # # #  #  #     # # # ##   #  ###  #  # # # #  #  ###  # "
    ];

    println!("Bir metin girin ? ");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_matches('\n').to_uppercase();
    let a_value='A' as u8;
    let z_value='Z' as u8;

    for r in 0..5 {
        for i in 0..t.len() {
            let b_value = t.as_bytes()[i];
            //println!("{}",b_value);
            if b_value >= a_value && b_value <= z_value {
                let start = ((b_value - a_value) * 4) as usize;
                let end = start + 4;

                for j in start..end {
                    let sharp = letter_rows[r].as_bytes()[j];
                    print!("{}", sharp as char);
                }
            }
        }
        println!();
    }
}