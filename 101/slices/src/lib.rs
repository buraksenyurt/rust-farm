#[cfg(test)]
mod tests {
    use crate::change_colors;

    #[test]
    fn simple_slice_tests() {
        let numbers = [1, 3, 5, 7, 9, 11, 13, 15];
        // the size for values of type `[{integer}]` cannot be known at compilation time"
        //let after_second = numbers[2..];
        let after_second = &numbers[2..]; //2nci indisten itibaren kalanını dilimle
        assert_eq!(after_second, [5, 7, 9, 11, 13, 15]);

        let between = &numbers[3..=6]; //3ncü indisten itibaren 6ncıya(dahil) kadar dilimle.
        assert_eq!(between, [7, 9, 11, 13]);
    }

    #[test]
    fn move_into_fn_test() {
        // string literallerden oluşan bir vector nesnesi
        let mut colors = vec![
            "red",
            "blue",
            "green",
            "orange",
            "purple",
            "dark blue",
            "ocean blue",
        ];
        // Değiştirilebilir referansını fonksiyona yolladık
        change_colors(&mut colors);
        // değişmiş versiyondaki ilk 3 rengi aldık
        let splited = &colors[..3];
        assert_eq!(splited, ["pink", "black", "magenta"]);
    }
}
// str tipi String Slice olarak da anılır. Mutable referansı alıp üstünde farazi değişiklik yaptık.
pub fn change_colors(slice: &mut [&str]) {
    slice[0] = "pink";
    slice[1] = "black";
    slice[2] = "magenta";
}
