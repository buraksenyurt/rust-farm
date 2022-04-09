mod content_type;
mod expression_data;
mod tag_type;

mod prelude{
    pub use crate::content_type::*;
    pub use crate::tag_type::*;
    pub use crate::expression_data::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
