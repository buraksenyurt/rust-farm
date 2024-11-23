use regex::Regex;
use crate::model::invoice::Invoice;

pub fn generate(template: &str, invoice: &Invoice) -> String {
    let mut rendered = template.to_string();

    let placeholders = [
        ("\\{\\{\\s*title\\s*\\}\\}", &invoice.title),
        ("\\{\\{\\s*customer\\s*\\}\\}", &invoice.customer),
        ("\\{\\{\\s*serial_number\\s*\\}\\}", &invoice.serial_number),
        (
            "\\{\\{\\s*total_amount\\s*\\}\\}",
            &invoice.total_amount.to_string(),
        ),
    ];

    for (placeholder, value) in &placeholders {
        let re = Regex::new(placeholder).unwrap();
        rendered = re.replace_all(&rendered, *value).to_string();
    }

    let re = Regex::new(r"\{\{\s*line_items\s*\}\}").unwrap();
    let items = invoice
        .line_items
        .iter()
        .map(|item| {
            format!(
                "<li>{} {} - {}$ ({})</li>",
                item.id, item.title, item.unit_price, item.quantity
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    rendered = re.replace_all(&rendered, items).to_string();

    rendered
}
