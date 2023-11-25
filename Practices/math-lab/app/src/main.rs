fn main() {
    let x = 4;
    let y = 5;
    let z = basic::add(x, y);
    println!("{x}+{y}={z}");

    println!(
        "Fiboannci serisindeki 10ncu sayı {}",
        algebra::fibonacci(10)
    );

    let collatz_series = algebra::collatz::get_sequence(10);
    collatz_series.iter().for_each(|n| println!("{n}"));

    for i in 1..=30 {
        println!("{}", algebra::fun::which_fizz_buzz(i));
    }
}
