/*
   Enum türü ile çalışmayı hatırlamak.

   Elektronik, market ve giyim ile ilgili veri yapıları tasarlamak istediğimiz düşünelim.
   OOP tarafında çalışanlar için Rust dünyasında ilke akla gelen seçenek Struct kullanmaktır.
   Ancak bazı hallerde enum türü de çok işe yarayabilir.
*/
fn main() {
    let raspy = Product::Electronics {
        name: "Raspberry 8Gb".to_string(),
        brand: "Raspberry".to_string(),
        warranty_years: 10,
    };
    println!("{:#?}", raspy);

    let c64_shirt = Product::Clothing {
        name: "Commodore 64 T-Shirt".to_string(),
        color: "Black".to_string(),
        size: "L".to_string(),
    };
    println!("{:#?}", c64_shirt);
}

// 00 altında belirtilen veri modelindeki her şey bir ürün ama özellikleri farklı.
// Bunları Product isimli bir enum sabitinde aşağıdaki gibi tasarlayabiliriz de.
#[derive(Debug)]
enum Product {
    Electronics {
        name: String,
        brand: String,
        warranty_years: u8,
    },
    Grocery {
        name: String,
        expiry_date: String,
    },
    Clothing {
        name: String,
        size: String,
        color: String,
    },
}

// 00 struct kullanarak model tasarımı
struct Electronic {
    name: String,
    brand: String,
    warranty_years: u8,
}
struct Grocery {
    name: String,
    expiry_date: String,
}
struct Clothing {
    name: String,
    size: String,
    color: String,
}

/*
   Enum türü kullanımı örnekleri farklılaştırabilir.
   Örneğin spor müsabakaları, bir oyundaki ya da eğitimdeki başarımlar, araç türleri vs
   Aşağıda birkaç farklı örnek de yer alıyor.
*/

enum SportEvent {
    Basketball {
        team1: String,
        team2: String,
        points: (u16, u16),
    },
    Volleyball {
        home_team: String,
        away_team: String,
        sets_won: (u8, u8),
    },
    TableTennis {
        player1: String,
        player2: String,
        sets_won: (u8, u8),
    },
    Today {
        title: String,
        what_happened: String,
        year: u8,
        month: u8,
        day: u8,
    },
}

enum Vehicle {
    Airplane {
        airline: String,
        flight_number: String,
        max_capacity: u8,
        max_altitude: u8,
    },
    Car {
        make: String,
        model: String,
        horse_power: u8,
        year: u16,
    },
    Bicycle {
        brand: String,
        gear_count: u8,
        color: String,
    },
    Boat {
        brand: String,
        range: u8,
        builder: String,
        capacity: u8,
    },
}
