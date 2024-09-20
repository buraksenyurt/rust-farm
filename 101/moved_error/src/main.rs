fn main() {
    let mut azon_bank = Bank::default();

    let mut meggy = Account::new(1013, "Meggy Ceyn Vatson".to_string());
    meggy.balance = 1000;
    // azon_bank.accounts.push(meggy);

    // Solution #01 (Clone)
    azon_bank.accounts.push(meggy.clone());

    // Case #00
    // print_account(meggy); // Value used after being moved [E0382]

    // Solution #01 (Clone)
    print_account(meggy);

    let mut john = Account::new(1094, "John Doe".to_string());
    john.balance = 950;
    azon_bank.accounts.push(john);

    // Solution #02 (Passed by Reference)
    azon_bank.accounts.iter().for_each(|a| print_account_ref(a));

    // Case #00
    // azon_bank.accounts.iter().for_each(|&a| print_account(a));
    /*
        azon_bank.accounts.iter().for_each(|&a| print_account(a));
       |                                         ^-
       |                                          |
       |                                          data moved here
       |                                          move occurs because `a` has type `Account`
       |                                        , which does not implement the `Copy` trait
    */
}

// Case #00
// fn print_account(account: Account) {
//     println!("{:?}", account);
// }

fn print_account(account: Account) {
    println!("{:?}", account);
}

// Solution #02
fn print_account_ref(account: &Account) {
    println!("{:?}", account);
}

// Case #00
// #[derive(Debug)]
// struct Account {
//     pub id: u32,
//     pub owner: String,
//     pub balance: u32,
// }

// Solution #01 (Clone trait)
#[derive(Debug, Clone)]
struct Account {
    pub id: u32,
    pub owner: String,
    pub balance: u32,
}

impl Account {
    pub fn new(id: u32, owner: String) -> Self {
        Account {
            id,
            owner,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    pub accounts: Vec<Account>,
}

impl Default for Bank {
    fn default() -> Self {
        Bank { accounts: vec![] }
    }
}
