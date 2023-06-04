use crate::model::prelude::*;

pub struct Unit {
    pub usings: Vec<Using>,
    pub namespace: Namespace,
    pub classes: Vec<Class>,
}
