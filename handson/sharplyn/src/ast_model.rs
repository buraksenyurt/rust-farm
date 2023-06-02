pub struct Method {
    pub name: String,
    pub return_type: String,
}

pub struct Field {
    pub name: String,
    pub f_type: String,
}

pub struct Property {
    pub name: String,
    pub p_type: String,
}

pub struct Namespace {
    pub name: String,
    pub classes: Vec<Class>,
}

pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}
