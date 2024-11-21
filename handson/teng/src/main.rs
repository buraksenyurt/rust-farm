use crate::file::*;
use crate::report::*;

mod file;
mod model;
mod report;

fn main() {
    let invoice_data = load_json("data_samples/invoice.json");
    let template_file = "templates/invoice.teng";
    let template = load_template(template_file);
    let output = generate(&template, &invoice_data);
    println!("{}", output);
}
