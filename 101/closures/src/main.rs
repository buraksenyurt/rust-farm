use std::f32::consts::PI;

fn main() {
    // i32 türünden değişken alıp onu 1 artıran bir fonksiyonu
    // increaser isimli bir değişkene atadık
    let increase = |v: i32| v + 1;
    println!(
        "7'ye bir eklersem tabii ki {} olur. Şaşılacak şey :P",
        increase(7)
    );

    // closure'lar aşağıdaki şekilde blok açılarak da yazılabilirler.
    let sum_all = |numbers: &Vec<i32>| -> i32 {
        let mut total = 0;
        for n in numbers.iter() {
            total += n;
        }
        total
    };
    let points = vec![1, 3, 5, 7, 9];
    println!("{:?} sayılarının toplamı {}.", points, sum_all(&points));

    // pek tabii closure'lara n sayıda parametrede gönderebiliriz
    let square = |a: i32, b: i32| 2 * (a + b);
    println!(
        "Domates bahçesini eni 3 ve boyu 5 metre ise çevresi {} metredir",
        square(3, 5)
    );

    // closure parametreleri generic tanımlanabilir ancak dikkat edilmesi gereken bir durum vardır.
    let moleculoid = |x| println!("{} moleküllerine ayrılacak", x);
    moleculoid(PI); // burada x parametresi f32 türüne bürünmüştür.
                    // Bu nedenle aşağıdaki satır hata verecektir.
                    // moleculoid(23); // expected `f32`, found integer
}
