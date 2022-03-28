fn main() {
    /*
       Version #1.
       Aşağıdaki kod derlenmeyecektir. Nitekim let ifadesi ilk if bloğu ile birlikte,
       &str türünden bir değer beklemekteyken son else bloğunda i32 dönüldüğü görülmektedir.

       Bu kodun çalışmadığını görünce i.ToString() denenebilir. Ama kod yine de derlenmeyecektir.

       `if` and `else` have incompatible types
    */
    // for i in 1..50 {
    //     let word = if i % 3 == 0 {
    //         "Fizz"
    //     } else if i % 5 == 0 {
    //         "Buzz"
    //     } else if i % 15 == 0 {
    //         "FizzBuzz"
    //     } else {
    //         i.to_string()
    //     };
    //     print!("{} ", word);
    // }

    /*
       Version #2.
       &str problemi için i değişkenini String'e dönüştürdükten sonra dereference edilmesi denenir.
       Dereference ve sonrasında string slice'ın elde edilmesi.
       Buna göre tasarlanan aşağıdaki kod parçası da derleme zamanı hatası verecektir.

       temporary value dropped while borrowed
    */
    for i in 1..50 {
        let word = if i % 3 == 0 {
            "Fizz"
        } else if i % 5 == 0 {
            "Buzz"
        } else if i % 15 == 0 {
            "FizzBuzz"
        } else {
            &*i.to_string()
        };
        print!("{} ", word);
    }
}
