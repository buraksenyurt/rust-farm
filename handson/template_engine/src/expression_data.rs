#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

impl ExpressionData {
    pub fn new(head: Option<String>, variable: String, tail: Option<String>) -> Self {
        Self {
            head,
            variable,
            tail,
        }
    }
}
