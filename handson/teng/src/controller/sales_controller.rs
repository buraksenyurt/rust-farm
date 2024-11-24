use crate::model::sales::SalesData;
use regex::Regex;

pub fn generate(template: &str, sales_data: &SalesData) -> String {
    let mut rendered = template.to_string();
    // println!("{:?}", sales_data);

    let placeholders = [
        ("\\{\\{\\s*report_title\\s*\\}\\}", &sales_data.report_title),
        ("\\{\\{\\s*month\\s*\\}\\}", &sales_data.month),
        (
            "\\{\\{\\s*total_sales\\s*\\}\\}",
            &sales_data.total_sales.to_string(),
        ),
    ];

    for (placeholder, value) in &placeholders {
        let re = Regex::new(placeholder).unwrap();
        rendered = re.replace_all(&rendered, *value).to_string();
    }

    let each_rx = Regex::new(r"\{\{\#each\s+(\w+)\s*\}\}([\s\S]*?)\{\{\/each\}\}").unwrap();
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
