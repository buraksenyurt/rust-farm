use std::f32::consts::PI;

// burada global bir variable tanımı söz konusu
static mut MIN_VALUE: u8 = 50;

fn main() {
    // en basit haliyle scope
    {
        let value = PI;
        println!("Pi değeri {}", value);
    } // Scope'un sonlandığı ve doğal olarak value değişkeninin öldüğü yer
      // println!("Bir kez daha pi değeri {}", value); // cannot find value `value` in this scope

    //println!("Sınıfı geçmek için sınavdan en az {} alınmalıdır",MIN_VALUE);
    // Use of mutable static is unsafe and requires unsafe function or block

    // Ancak ısrarcı isek güvensiz bir blok açıp global değişkeni bu kapsamda kullanabiliriz.
    unsafe {
        println!("Sınıf geçmek için en az {} alınmalıdır", MIN_VALUE);
        MIN_VALUE = 70;
        println!("Sınıf geçme notu {} olarak değiştirilmiştir", MIN_VALUE);
    }
}
