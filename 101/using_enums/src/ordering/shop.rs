use super::product::Product; // Super is ref to parent model (in this case 'ordering')

#[derive(Debug)]
pub struct Shop {
    products: Vec<Product>,
}

impl Shop {
    pub fn new() -> Self {
        Self { products: vec![] }
    }

    pub fn add(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn get(&self, index: usize) -> Option<&Product> {
        if self.products.len() > index {
            Some(&self.products[index])
        } else {
            None
        }
    }
}
