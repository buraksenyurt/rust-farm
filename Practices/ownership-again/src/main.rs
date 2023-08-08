fn main() {
    // // #1nci Örnek
    // {
    //     let nick_name = String::from("Dam Van Dam");
    //     // nick_name ve amount değişkenleri ve sahip oldukları değerler sadece bu scope içinde yaşarlar
    //     let amount = 1000;
    // }
    // // Burada nick_name'in ve amount'un sahip olduğu değerler düşürülür
    // // ve deallocation yapılır
    // println!("Nick name is '{nick_name}' and amount is {amount}");

    // 2nci Örnek (Value moved occurs here)
    let point_1 = 10;
    let mut point_2 = point_1; // i32'nin boyutu bellidir, Deep Copy olur. point_2 değeri point_1'e kopyalanır.
    println!("P1={point_1} , P2={point_2}");
    point_2 = 15; // point_2 de yapılan değişiklik point_1'i etkilemez. 10 ve 15 değerlerinin sahipleri farklıdır.
    println!("P1={point_1} , P2={point_2}");

    // Ama aşağıdaki duruma bakarsak
    let my_name = String::from("Clot Van Damme");
    let your_name = my_name.clone(); // Clot Van Damme'ın sahipliği el değiştirir ve your_name'e geçer. my_name artık kullanılamaz.
                                     // Buna göre aşağıdaki kullanım borrow of moved value hatasının doğmasına neden olur
    println!("My name is {my_name} and your name is {your_name}");
    // elbette your_name.clone() ile bu hatanın önüne geçebiliriz.
    // Bu durumda Deep Copy yapılıp her iki değişken de kullanılabilir.
    // Lakin bilindiği üzere deep copy operasyonu pahalıdır.
}
