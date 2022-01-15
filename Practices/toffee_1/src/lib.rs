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
}
