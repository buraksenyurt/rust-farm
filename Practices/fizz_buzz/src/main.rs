mod fermat;

fn main() {
    /*
       Version #1.
       Aşağıdaki kod derlenmeyecektir. Nitekim let ifadesi ilk if bloğu ile birlikte,
       &str türünden bir değer beklemekteyken son else bloğunda i32 dönüldüğü görülmektedir.

       Bu kodun çalışmadığını görünce i.ToString() denenebilir. Ama kod yine de derlenmeyecektir.

       `if` and `else` have incompatible types
    */
    // for i in 1..50 {
    //     let word = if i % 15 == 0 {
    //         "FizzBuzz"
    //     } else if i % 3 == 0 {
    //         "Fizz"
    //     } else if i % 5 == 0 {
    //         "Buzz"
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
    // for i in 1..50 {
    //     let word = if i % 15 == 0 {
    //         "FizzBuzz"
    //     } else if i % 3 == 0 {
    //         "Fizz"
    //     } else if i % 5 == 0 {
    //         "Buzz"
    //     } else {
    //         &*i.to_string()
    //     };
    //     print!("{} ", word);
    // }

    /*
       Version #3.

       i değeri için ayrı bir değişken tanımlanır.
       to_string() ile i bu değişkende tutulur.
       i_value değişkeninin ömrü &str referansının kullanılması için yeterlidir.

       Kod sorunsuz derlenip çalışacaktır.
    */
    // for i in 1..50 {
    //     let i_value;
    //     let word = if i % 15 == 0 {
    //         "FizzBuzz"
    //     } else if i % 3 == 0 {
    //         "Fizz"
    //     } else if i % 5 == 0 {
    //         "Buzz"
    //     } else {
    //         i_value = i.to_string();
    //         &*i_value
    //     };
    //     print!("{} ", word);
    // }

    /*
       Version #4.

       Bu sefer sayının fizz buzz veya fizzbuzz olma durumunu bir fonksiyona devrediyoruz.
       Lakin missing lifetimes specifier hatasına takılacağız.

       Version #5.
       Sorunu çözmek için 'a ile lifetime açık bir şekilde belirtilir. Fakat bu sefer
       i_value değişkeni yan çizer. Nitekim i_value'nun sahiplendiği bir değere ait referans,
       fonksiyondan geriye döndürülmeye çalışılmaktadır. Oysa ki değişkenin zaten fonksiyon
       içerisinde bir sahibi bulunmakta. Rust bu duruma müsaade etmez.

       returns a value referencing data owned by the current function
    */
    // for i in 1..50 {
    //     print!("{} ", check(i));
    // }

    /*
       Version #6

       Enum veri türü destekli Fizz Buzz çözümü.
       Bu sefer sayı durumunu String veya &str ile değil bir enum veri yapısı ile temsil ediyoruz.
    */
}

// Version #4
// fn check(i: i32) -> &str {
//     let i_value;
//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else {
//         i_value = i.to_string();
//         &*i_value
//     }
// }

// Version #5
// fn check<'live_long_and_well>(i: i32) -> &'live_long_and_well str {
//     let i_value;
//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else {
//         i_value = i.to_string();
//         &*i_value
//     }
// }
