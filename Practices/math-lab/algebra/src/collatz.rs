pub fn get_sequence(mut number: u32) -> Vec<u32> {
    let mut sequence = vec![number];

    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = 3 * number + 1;
        }
        sequence.push(number);
    }

    sequence
}
