use crate::traits::Reportable;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SalesData {
    pub report_title: String,
    pub month: String,
    pub total_sales: u32,
    pub sales_by_category: Vec<CategorySales>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategorySales {
    pub category: String,
    pub amount: u32,
}

impl Reportable for SalesData {
    fn generate(template: &str, sales_data: &Self) -> String {
        let mut rendered = template.to_string();

        let placeholders = [
            ("\\{\\{\\s*report_title\\s*\\}\\}", &sales_data.report_title),
            ("\\{\\{\\s*month\\s*\\}\\}", &sales_data.month),
            (
                "\\{\\{\\s*total_sales\\s*\\}\\}",
                &sales_data.total_sales.to_string(),
            ),
        ];

        for (placeholder, value) in &placeholders {
            let rex = Regex::new(placeholder).unwrap();
            rendered = rex.replace_all(&rendered, *value).to_string();
        }

        let each_rx = Regex::new(r"\{\{#each\s+(\w+)\s*}}([\s\S]*?)\{\{/each}}").unwrap();
        rendered = each_rx
            .replace_all(&rendered, |caps: &regex::Captures| {
                let key = &caps[1];
                let template_block = &caps[2];

                if key == "sales_by_category" {
                    sales_data
                        .sales_by_category
                        .iter()
                        .map(|item| {
                            let mut block = template_block.to_string();
                            block = block.replace("{{ category }}", &item.category);
                            block = block.replace("{{ amount }}", &item.amount.to_string());
                            block
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                } else {
                    String::new()
                }
            })
            .to_string();

        rendered
    }
}
