fn main() {
    let azoncoin = BitCredit { btc_number: 1234 };
    azoncoin.pay(100.);

    let maslak_acct = DebitCard {
        owner: "Toni Burosso".to_string(),
        id: 12,
        account_number: "MSLK-12045-1234".to_string(),
    };
    maslak_acct.pay(125.);
}

struct MasterCard {
    id: u8,
    owner: String,
    is_valid: bool,
}

struct VisaCard {
    id: u8,
    owner: String,
}

struct DebitCard {
    id: u8,
    owner: String,
    account_number: String,
}

impl Payment for DebitCard {
    fn pay(&self, amount: f32) -> bool {
        println!(
            "{} numaralı karttan {} TL ödeme alınacak",
            self.account_number, amount
        );
        true
    }
}

struct BitCredit {
    btc_number: u32,
}

impl Payment for BitCredit {
    fn pay(&self, amount: f32) -> bool {
        println!(
            "{} için BitCredit ile ödeme alınacak. Tutar {}",
            self.btc_number, amount
        );
        true
    }
}

trait Payment {
    fn pay(&self, amount: f32) -> bool;
}
