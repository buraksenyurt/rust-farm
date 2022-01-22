#[cfg(test)]
mod tests {
    use crate::{find_square_less_than, process};

    #[test]
    fn process_test() {
        // örneğin alttaki vektörün her bir elemanı üstünde
        let mut points = vec![1, 2, 3, 4];
        // ikinci parametre ile gönderdiğimiz fonksiyonu çağırabiliriz
        process(&mut points, |n| n * n);
        assert_eq!(points, [1, 4, 9, 16]);
    }

    #[test]
    fn built_in_hof_test() {
        let max = 1000;
        let result = find_square_less_than(max);
        assert_eq!(result, 16);

        // Yukarıda test ettiğimiz fonksiyon içerisindeki işlerin aynısını,
        // Rust'ın built-in higher order function'ları yardımıyla daha kolay yapabiliriz.
        let result = (0..)
            .map(|n| n * n)
            .take_while(|&n| n < max)
            .filter(|n| n % 2 == 0)
            .fold(0, |count, _| count + 1);
        assert_eq!(result, 16);
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

// Bu kobay fonksiyonda karesi üst limitten küçük kaç rakam var bulmaya çalışıyoruz.
pub fn find_square_less_than(max: i32) -> i32 {
    let mut count = 0;
    for i in 0.. {
        let square = i * i;
        if square > max {
            break;
        } else if i % 2 == 0 {
            count += 1;
        }
    }
    count
}
