/*
   Fizz Buzz'ın farkl şekillerde çözümü
*/
fn main() {
    execute_v2();
    // execute_v1();
}

fn execute_v2() {
    // Version 2 - pattern matching ile
    for num in 1..=50 {
        match (num % 3 == 0, num % 5 == 0) {
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (true, true) => println!("FizzBuzz"),
            _ => println!("{num}"),
        }
    }
}
fn execute_v1() {
    // Version 1 - if ile
    for num in 1..=50 {
        if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{num}");
        }
    }
}
