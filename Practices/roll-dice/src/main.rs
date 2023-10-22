use rand::Rng;
use std::str::FromStr;

/*
   Zar atma oyunu.
   Her seferinde tek bir zar atılır. 1den 6ya kadar bir değer olabilir.
   Deneme sayısına göre rastgele zar değerleri üretilir.
   Denemelerde gelen zara göre bir toplam değer bulunur.
   Örneğin 6d3 , 6 kenarlı standart bir zar ile 3 deneme atışını işaret eder.
*/

fn main() {
    let try_one = "6d3".parse::<Roll>().unwrap();
    let total = try_one.apply();
    println!("{:?}. Total is {}", try_one, total);
}

#[derive(Debug)]
struct Roll {
    dice: u16,
    sides: u16,
}

impl Roll {
    fn apply(&self) -> u32 {
        let mut rng = rand::thread_rng();
        (0..self.dice)
            .map(|_| rng.gen_range(1..=self.sides) as u32)
            .sum()
    }
}

#[derive(Debug)]
struct RollError;

impl FromStr for Roll {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let specs: Vec<&str> = s.trim().split('d').collect();
        let sides = specs[0].parse::<u16>().map_err(|_| RollError);
        let dice = specs[1].parse::<u16>().map_err(|_| RollError);

        Ok(Self {
            sides: sides.unwrap(),
            dice: dice.unwrap(),
        })
    }
}
