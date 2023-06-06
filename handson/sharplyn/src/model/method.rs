use crate::model::prelude::*;

pub struct Method {
    pub name: String,
    pub return_type: SharpType,
    pub parameters: Vec<Parameter>,
}
