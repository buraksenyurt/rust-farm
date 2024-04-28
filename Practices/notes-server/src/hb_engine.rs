use crate::utility::get_file_path;
use handlebars::Handlebars;
use std::sync::Arc;

pub async fn create_handlebars<'a>() -> Arc<Handlebars<'a>> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("index", get_file_path("templates/index.hbs"))
        .unwrap();
    handlebars
        .register_template_file("noteForm", get_file_path("templates/noteForm.hbs"))
        .unwrap();
    handlebars
        .register_template_file("list", get_file_path("templates/list.hbs"))
        .unwrap();
    handlebars
        .register_template_file("ordered", get_file_path("templates/list.hbs"))
        .unwrap();
    handlebars
        .register_template_file("detail", get_file_path("templates/detail.hbs"))
        .unwrap();
    handlebars
        .register_template_file("error", get_file_path("templates/error.hbs"))
        .unwrap();

    Arc::new(handlebars)
}
