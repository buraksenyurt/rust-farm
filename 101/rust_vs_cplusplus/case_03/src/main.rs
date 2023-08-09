fn main() {
    let some_data = 1;
    let mut buffer = [0; 10]; // 10 elemanlı bir dizi oluşturuluyor

    for i in 0..=11 {
        buffer[i] = 5; // Dizi sınırlarını aşma girişimi, Rust derleyicisi tarafından yakalanır
    }

    println!("some_data = {}", some_data);
}
