mod content_type;
mod expression_data;
mod tag_type;
pub mod utility;

pub mod prelude {
    pub use crate::content_type::*;
    pub use crate::expression_data::*;
    pub use crate::tag_type::TagType::*;
    pub use crate::tag_type::*;
    pub use crate::utility::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn should_literal_works_test() {
        let sample = "<b>Hello there!</b>";
        assert_eq!(
            ContentType::Literal(sample.to_string()),
            get_content_type(sample)
        )
    }

    #[test]
    fn should_template_variable_works_test() {
        let content = ExpressionData {
            head: Some("Melaba ".to_string()),
            variable: "username".to_string(),
            tail: Some(" .Nasılsın?".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Melaba [[username]] .Nasılsın?")
        );
    }

    #[test]
    fn should_if_tag_works_test() {
        assert_eq!(
            ContentType::Tag(If),
            get_content_type("[@ if username == 'obiwan' @]")
        );
    }

    #[test]
    fn should_loop_tag_works_test() {
        assert_eq!(
            ContentType::Tag(Loop),
            get_content_type("[@ loop category in categories @]")
        );
    }

    #[test]
    fn should_symbol_pairs_works_test() {
        let expected = true;
        let actual = check_matching_pair("[[username]]", "[[", "]]");
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_symbol_checks_works_test() {
        let expected = true;
        let actual = check_symbol("[[username]]", "[[");
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_get_index_for_symbol_works_test() {
        let expected = (true, 7);
        let actual = get_index_for_symbol("Melaba [[username]] .", '[');
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_get_expression_data_works_test() {
        let expected = ExpressionData::new(
            Some("Melaba ".to_string()),
            "username".to_string(),
            Some(" .Nasılsın?".to_string()),
        );
        let data = "Melaba [[username]] .Nasılsın?";
        let actual = get_expression_data(data);
        assert_eq!(actual, expected);
    }
}
