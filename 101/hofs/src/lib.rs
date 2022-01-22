#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn process_test() {
        // örneğin alttaki vektörün her bir elemanı üstünde
        let mut points = vec![1, 2, 3, 4];
        // ikinci parametre ile gönderdiğimiz fonksiyonu çağırabiliriz
        process(&mut points, |n| n * n);
        assert_eq!(points, [1, 4, 9, 16]);
    }
}

// Kendi yazdığımız basit bir Higher Order Function.
// process isimli fonksiyon generic F parametresi almakta
// F ise i32 türünden parametre alıp geriye i32 döndüren bir fonksiyonu işaret etmekte.
// Dolayısıyla process fonksiyonuna mutable bir vecktör gönderip vu vektör'ün herbir elemanı üstünde
// func ile gelen fonksiyonu çalıştırabiliriz.
pub fn process<F>(numbers: &mut Vec<i32>, func: F)
where
    F: Fn(i32) -> i32,
{
    // Vektördeki tüm elemanlar için func ile gelen fonksiyon çağırılıyor ve o eleman değiştiriliyor
    let mut i = 0;
    while i < numbers.len() {
        let nv = func(numbers[i]);
        numbers[i] = nv;
        i += 1;
    }
}
