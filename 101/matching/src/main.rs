/*
   match ile kullanılan ifadeler diğer dillerdeki switch veya when kullanımlarına benzer.
*/
use crate::Status::{Completed, ErrorState, Started, Waiting};

fn main() {
    check_status(Status::Started);
    check_status(Status::Waiting);
    check_status(Status::Completed);
    check_status(Status::ErrorState);

    let codes = vec![1, 44, 90, -9, 345];
    for c in codes {
        check_shipment(c);
    }

    // Bir tuple türü içinde match ifadesi kullanılabilir
    let points = [(0, 0), (4, 0), (0, 8), (32, 45)];
    for point in points {
        match point {
            (0, 0) => println!("Merkezde."),
            (x, 0) => println!("X ekseninde ({},0)", x),
            (0, y) => println!("Y ekseninde (0,{})", y),
            (x, y) => println!("X,Y koordinatlarında ({},{})", x, y),
        };
    }
}

// İlk kullanımda Status enum türünün olası durumları ele alınır.
fn check_status(s: Status) {
    match s {
        Started => println!("Başlatıldı"),
        Waiting => println!("Beklemede"),
        Completed => println!("Tamamlandı"),
        ErrorState => println!("Hata durumunda"),
    }
}

enum Status {
    Started,
    Waiting,
    Completed,
    ErrorState,
}

// code değerine göre match kullanımı
// 1,44,90 gibi kesin rakamlar verilebileceği gibi
// 2..=999 gibi geniş bir aralık da verilebilir
// _ ile de diğer kollarda belirtilenler haricindeki bir olasılık ele alınır.
fn check_shipment(code: i32) {
    let country = match code {
        1 => "USA",
        44 => "United Kingdom",
        90 => "Turkey",
        2..=999 => "Not Available",
        _ => "Not Valid",
    };
    println!("{}\t:\t{}", code, country);
}
