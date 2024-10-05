use std::cell::RefCell;
use std::rc::Rc;

/*
   Interior Mutability için RefCell ile Rc kullanılabilir.
   Rc(Reference Counting Pointer) tek thread'ler için idealdir
   RefCell thread safe değildir ve thread safe önemli ise Mutex tercih edilebilir.
*/

#[derive(Debug)]
struct Client<'a> {
    name: &'a str,
    amount: f32,
}
fn gain(client: &Rc<RefCell<Client>>) {
    let mut client = client.borrow_mut();
    client.amount += 10_f32;
}

#[test]
fn gain_test() {
    let gandalf = Rc::new(RefCell::new(Client {
        name: "gandalf",
        amount: 100.0,
    }));
    gain(&gandalf);
    dbg!(&gandalf);
    gain(&gandalf);
    dbg!(&gandalf);
    gain(&gandalf);
    dbg!(&gandalf);
}
