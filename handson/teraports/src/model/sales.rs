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