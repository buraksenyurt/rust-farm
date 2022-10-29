use crate::category::Category;
use crate::product::Product;
use crate::template::CATEGORY_TEMPLATE;
use axum::extract::Path;
use axum::response::Html;
use minijinja::render;

/*
URL üstünden kategori adı ne gelirse gelsin örnek bir veri setini
CATEGORY_TEMPLATE içerisindeki metin ile çarpıştırıp döndüren bir fonksiyondur.
*/
pub async fn get_category(Path(category_name): Path<String>) -> Html<String> {
    let product_list = vec![
        Product {
            id: 1001,
            name: "Rust 101".to_string(),
            list_price: 15.0,
        },
        Product {
            id: 1002,
            name: "Anti Patterns".to_string(),
            list_price: 44.59,
        },
        Product {
            id: 1003,
            name: "How Linux Works".to_string(),
            list_price: 23.99,
        },
    ];
    let c = Category {
        id: 1,
        title: category_name.to_string(),
        products: product_list,
    };
    let r = render!(CATEGORY_TEMPLATE, category => c);
    Html(r)
}
