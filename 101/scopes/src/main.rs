// burada global bir variable tanımı söz konusu
static mut MIN_VALUE: u8 = 50;

fn main() {
    // en basit haliyle scope
    {
        let value = 3.14;
        println!("Pi değeri {}", value);
    } // Scope'un sonlandığı ve doğal olarak value değişkeninin öldüğü yer
    // println!("Bir kez daha pi değeri {}", value); // cannot find value `value` in this scope
}
