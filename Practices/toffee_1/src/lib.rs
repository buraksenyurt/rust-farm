#[cfg(test)]
mod tests {
    #[test]
    fn very_simple_closure_test() {
        let triple = |x| x * x;
        let result = triple(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn simple_iter_test() {
        // Alttaki vektörde yer alan tuple türlerindeki ilk haneyi ikiye
        // ve ikinciyi üçe katlayıp değiştirecek kodu yazalım
        let values = vec![(0, 1), (2, 3), (3, 4)];
        // _ ile türün tahminini derleyiciye bıraktık
        let new_values: Vec<(_, _)> = values
            .iter()
            //.map(|(a, b)| (a *2, b*3))
            .map(|t| (t.0 * 2, t.1 * 3))
            .collect();
        assert_eq!(new_values, [(0, 3), (4, 9), (6, 12)]);
    }

    #[test]
    fn simple_iter_mut_test() {
        // Alttaki sayıların herbirinin küpünü alıp vektörü değiştirecek kodu yazalım.
        let mut numbers = vec![1, 2, 3, 4, 5];

        numbers.iter_mut().for_each(|n| *n *= 3);
        assert_eq!(numbers, [3, 6, 9, 12, 15]);

        // Bir başka versiyonu
        let mut numbers = vec![1, 2, 3, 4, 5];
        for n in numbers.iter_mut() {
            *n *= 3;
        }
        assert_eq!(numbers, [3, 6, 9, 12, 15]);

        // Bir başka versiyonu daha
        let mut numbers = vec![1, 2, 3, 4, 5];
        for n in &mut numbers {
            // syntactic sugar form
            *n *= 3;
        }
        assert_eq!(numbers, [3, 6, 9, 12, 15]);
    }

    #[test]
    fn simple_filter_map_collect_test() {
        // Aşağıdkai vektörde yer alan cümlelerden içinde 'mavi' geçeni/geçenleri çekip
        // büyük harfe dönüştürülmüş hallerini yeni bir vektörde toplayalım.
        let words = vec![
            "kırmızı elmayı çok severim",
            "onun sakalı resmen kızıl",
            "mavi okyanuslarda yüzesim var",
            "arabanın rengi kırmızı",
            "masmavi gökyüzüne bakmak ne güzel",
            "kara lahana çorbasına bayılırım",
            "yemekte kırmızı turp var",
        ];
        // _ ile türün tahminini derleyiciye bıraktık
        /*
        let result: Vec<_> = words
            .into_iter()
            .filter(|w| w.contains("mavi"))
            .map(|f| f.to_uppercase())
            .collect();
        */
        let result = words
            .into_iter()
            .filter(|w| w.contains("mavi"))
            .map(|f| f.to_uppercase())
            .collect::<Vec<_>>(); // Turbo Fish versiyonu
        assert_eq!(
            result,
            [
                "MAVI OKYANUSLARDA YÜZESIM VAR",
                "MASMAVI GÖKYÜZÜNE BAKMAK NE GÜZEL"
            ]
        );
    }
}
