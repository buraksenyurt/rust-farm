#[derive(Debug)]
pub enum Product {
    Electronics {
        // Variant
        name: String,
        brand: String,
        warranty_years: u8,
    },
    Grocery {
        // Variant
        name: String,
        expiry_date: String,
    },
    Clothing {
        // Variant
        name: String,
        size: String,
        color: String,
    },
    DiscountCoupon(f32), // Unlabeled Field
    OutOfService,
}

impl Product {
    pub fn summary(&self) -> String {
        match self {
            Product::Electronics {
                name,
                brand,
                warranty_years,
            } => {
                format!(
                    "{} ({}). Warranty duration is {} years",
                    name, brand, warranty_years
                )
            }
            Product::Grocery { name, expiry_date } => {
                format!("{}. Last usage date is '{}'", name, expiry_date)
            }
            Product::Clothing { name, size, color } => {
                format!("{} ({}) color is {}", name, size, color)
            }
            Product::DiscountCoupon(amount) => {
                format!("You have {} coin discount", amount)
            }
            Product::OutOfService => "Vacation time. The shop is closed!".to_string(),
        }
    }
}
