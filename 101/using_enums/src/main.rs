use ordering::product::Product;
use ordering::shop::Shop;

mod ordering;

fn main() {
    let raspy = Product::Electronics {
        name: "Raspberry 8Gb".to_string(),
        brand: "Raspberry".to_string(),
        warranty_years: 10,
    };
    println!("{}", raspy.summary());

    let c64_shirt = Product::Clothing {
        name: "Commodore 64 T-Shirt".to_string(),
        color: "Black".to_string(),
        size: "L".to_string(),
    };

    println!("{}", c64_shirt.summary());

    let nugat = Product::Grocery {
        name: "Nugat".to_string(),
        expiry_date: "01.01.2004".to_string(),
    };
    println!("{}", nugat.summary());

    let mut your_market = Shop::new();
    your_market.add(nugat);
    your_market.add(c64_shirt);
    your_market.add(raspy);
    your_market.add(Product::DiscountCoupon(40.50));
    println!("{:#?}", your_market);

    let index = 9000;
    let out_of_stock = Product::OutOfService;
    let product = your_market.get(index).unwrap_or(&out_of_stock);
    println!("{}", product.summary());
}
