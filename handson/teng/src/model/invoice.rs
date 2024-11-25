use crate::traits::Reportable;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub unit_price: f32,
    pub quantity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub serial_number: String,
    pub customer: String,
    pub total_amount: f32,
    pub line_items: Vec<Item>,
}

impl Reportable for Invoice {
    fn generate(template: &str, invoice: &Self) -> String {
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

        let rex = Regex::new(r"\{\{\s*line_items\s*}}").unwrap();
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

        rendered = rex.replace_all(&rendered, items).to_string();

        rendered
    }
}
