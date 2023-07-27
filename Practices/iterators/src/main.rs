fn main() {
    // //#1 Aşağıdaki kod parçasını ele alalım
    let mut numbers = vec![2, 4, 6, 8];
    // let mut i = 0;
    // let n = numbers.len();
    //
    numbers = vec![1, 3, 5]; // Burada numbers vektörünü yeniden bağladık
    // while i != n {
    //     println!("{}", numbers[i]);
    //     i += 1;
    // }
    // #1 deki kod bloğunu çalıştırdığımızda çalışma zamanında hata alırız ve program kırılır.
    /*
    cargo run
    Compiling iterators v0.1.0 (/home/buraks/development/rust-farm/Practices/iterators)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/iterators`
        1
        3
        5
        thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3', src/main.rs:9:24
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

     */
    // #2 Iterator kullanılan fonksiyonlarda ise bu hata oluşmaz.
    numbers.iter().for_each(|n| println!("{}", n));
}
