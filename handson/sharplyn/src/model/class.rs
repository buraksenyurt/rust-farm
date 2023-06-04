use crate::model::prelude::*;

pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}
