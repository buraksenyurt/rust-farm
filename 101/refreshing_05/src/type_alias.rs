/*
   Bazen karmaşık bir parametre yapısı için Type Alias'ı kullanılabilir.
   Örneğin add_materials metodunda kullanılan Material tipi aslında
   Option<(str,u8,u8)> için bir takma isim belirtmektedir.

   dbg! makro çıktılarını görmek için programı komut satırından
   cargo test -- --nocapture
   şeklinde çalıştırmalıyız.
*/
type Material<'a> = Option<(&'a str, u8, u8)>;

fn add_materials(material: Material) {
    if let Some(m) = material {
        dbg!(m.0);
        dbg!(m.1);
        dbg!(m.2);
    }
    dbg!("There are no materials\n");
}

#[test]
fn test_type_alias() {
    add_materials(Material::None);
    add_materials(Material::Some(("Hello Again", 10, 100)));
}
