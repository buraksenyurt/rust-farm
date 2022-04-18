fn main() {
    let x = 3;
    let y = 5;
    let total = sum(x, y);
    println!("Total is {}", total);
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
