#[cfg(test)]
mod test {
    #[test]
    fn should_ast_works() {
        let a = 1;
        let b = 2;
        let c = a + b;
        assert_eq!(a, 3);
    }
}
