use crate::content_type::ContentType;

mod content_type;
mod expression_data;
mod tag_type;

mod prelude {
    pub use crate::content_type::*;
    pub use crate::expression_data::*;
    pub use crate::get_content_type;
    pub use crate::tag_type::TagType::*;
    pub use crate::tag_type::*;
}

pub fn get_content_type(_data: &str) -> ContentType {
    todo!()
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
            tail: Some(" . Nasılsın?".to_string()),
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
            get_content_type("[@ for category in categories @]")
        );
    }
}
