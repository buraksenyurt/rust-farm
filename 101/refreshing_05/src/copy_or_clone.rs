use std::ops::Add;

#[test]
fn move_position_test() {
    /*
       Çalıştırmak için;
       cargo test -- --nocapture move_position_test

       Aşağıdaki senaryoda
       move occurs because `tank_position` has type `Position`, which does not implement the `Copy` trait
       hatası alınır. Nitemk ilk dbg! çağrısı tank_position değişkeninin sahipliğini alır ve
       dolayısıyla sonraki satırda tank_position var olmaz.

       Burada pratik yollardan birisi Clone trait'i uygulanıp clone metodu açıkça çağırmaktır.
    */
    // let mut tank_position = Position { x: 10.0, y: 10.0 };
    // dbg!(tank_position);
    // tank_position.x += 1.0;
    // dbg!(tank_position);

    let mut tank_position = PositionV2 { x: 10.0, y: 10.0 };
    dbg!(tank_position.clone());
    tank_position.x += 1.0;
    dbg!(tank_position);
}
// #[derive(Debug)]
// struct Position {
//     pub x: f32,
//     pub y: f32,
// }

#[derive(Debug, Clone)]
struct PositionV2 {
    pub x: f32,
    pub y: f32,
}

impl Add for PositionV2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn add_position_with_clone_test() {
    /*
       Aşağıdaki senaryoda PositionV2'ler toplanmaya çalışmaktadır.
       Ancak clone kullanmadığımız durumda Move ihlali söz konusu olacağından
       position2 satırı çalışmaz. Açıkça clone çağrısı yapılabilir.
       Ancak clone çaprısını position değişkeninin atanmasında da kullanmak gerekir.
       Bunun daha pratik bir çözümü vardır. Sürekli clone fonksiyonu kullanmak yerine,
       Copy trait'i kullanılabilir. V3 sürümünde bu durum ele alınmaktadır.
    */
    let tank_position = PositionV2 { x: 10.0, y: 10.0 };
    let target_position = PositionV2 { x: 5.0, y: 2.0 };
    // let position = tank_position + target_position;
    //  move occurs because `target_position` has type `PositionV2`, which does not implement the `Copy` trait
    // let position2 = tank_position + target_position;
    let position = tank_position.clone() + target_position.clone();
    let position3 = tank_position.clone() + target_position.clone();
}

#[derive(Debug, Clone, Copy)]
struct PositionV3 {
    pub x: f32,
    pub y: f32,
}

impl Add for crate::copy_or_clone::PositionV3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn add_position_with_copy_trait_test() {
    let tank_position = PositionV3 { x: 10.0, y: 10.0 };
    let target_position = PositionV3 { x: 5.0, y: 2.0 };
    // Copy trait'i kullandığımız için açıkça clone çağrısı yapmamıza gerek kalmadı
    let position = tank_position + target_position;
    let position2 = tank_position + target_position;
}
