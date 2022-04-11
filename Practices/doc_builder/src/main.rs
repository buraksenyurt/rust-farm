use doc_builder::{Bottom, Document, LineItems, Product, Title};

fn main() {
    // Bir Document nesnesi oluşturuyoruz
    let mut invoice = Document { sections: vec![] };
    // Dokümana eklenecek bir Title nesnesi örnekliyoruz.
    let title = Title::new(
        "Burağın Retro Bilgisayar Dükkanı".to_string(),
        "Fatura. 11.03.2003".to_string(),
    );
    // Title'ı
    invoice.add(Box::new(title));

    let mouse = Product::new(1, "Locitek Kablolu Mouse".to_string(), 95.50, 1);
    let keyboard = Product::new(2, "Eyç Pi Q Klavye.".to_string(), 150.00, 1);
    let mut line_items = LineItems::default();
    line_items.add(mouse);
    line_items.add(keyboard);

    invoice.add(Box::new(line_items));

    let bottom = Bottom::new("İletişim numarası : 23 23 23".to_string());
    invoice.add(Box::new(bottom));

    invoice.print();
}
