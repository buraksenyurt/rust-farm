/*
   Fizz Buzz'ın farkl şekillerde çözümü
*/
fn main() {
    //execute_v4();
    execute_v3();
    //execute_v2_point_5();
    //execute_v2();
    // execute_v1();
}

fn execute_v4() {
    // Version 4 - fonksiyonel yaklaşım
    //
    //
    // let _ gibi bir atama yapmadan aşağıdakini kullanırsak
    // note: iterators are lazy and do nothing unless consumed
    // şeklinde bir uyarı mesajı alırız
    // (1..=50).map(|num| match (num % 3 == 0, num % 5 == 0) {
    //     (true, false) => println!("Fizz"),
    //     (false, true) => println!("Buzz"),
    //     (true, true) => println!("FizzBuzz"),
    //     _ => println!("{num}"),
    // });

    (1..=50).for_each(|num| match (num % 3 == 0, num % 5 == 0) {
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (true, true) => println!("FizzBuzz"),
        _ => println!("{num}"),
    });
}

fn execute_v3() {
    // Version 3- pattern matching ile ama bu sefer String kullanımımız farklı
    let mut output = String::new();
    for num in 1..=50 {
        match (num % 3 == 0, num % 5 == 0) {
            (true, false) => output.push_str("Fizz"),
            (false, true) => output.push_str("Buzz"),
            (true, true) => output.push_str("FizzBuzz"),
            _ => output.push_str(&num.to_string()),
        };
        // Windows işletim sistemindeyse farklı satır sonu basılması için aşağıdaki kontrol eklenir
        if cfg!(windows) {
            output.push_str("\r\n");
        }
        if cfg!(not(windows)) {
            output.push('\n');
        }
    }
    println!("{output}");
}

fn execute_v2_point_5() {
    // Version 2.5 - pattern matching ile ama bu to_string kullanarak
    // to_string bir çözümdür ama bilindiği üzere her to_string çağrısı
    // bellekte yeni bir String allocation anlamına gelir.
    for num in 1..=50 {
        let result = match (num % 3 == 0, num % 5 == 0) {
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (true, true) => "FizzBuzz".to_string(),
            _ => num.to_string(),
        };

        println!("{result}");
    }
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
