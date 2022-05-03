use crate::prelude::*;

pub fn init_first_level(world: &mut World) {
    // MAP sabitindeki karakter setini kullanacağız.
    // Önce satır satır ayırıyoruz
    let rows: Vec<&str> = MAP.trim().split('\n').map(|r| r.trim()).collect();

    // Satırlardaki kolonları almamız lazım
    for (y, row) in rows.iter().enumerate() {
        // Boşluk karakterine göre ayrıştırıp columns isimli vector'e alıyoruz
        let columns: Vec<&str> = row.split(' ').collect();
        // Şimdi her bir kolonu dolaşabilir x ve y indekslerini kullanabiliriz
        for (x, column) in columns.iter().enumerate() {
            // x, y değerlerini kullanarak bir konum bileşeni oluşturuyoruz.
            let position = Position::new(x as u8, y as u8, 0);
            // İlgili pozisyondaki karakterin ne olduğuna bakıyoruz ve ona göre bir nesne üretiyoruz
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position)
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position)
                }
                "S" => {
                    create_floor(world, position);
                    create_chest_spot(world, position)
                }
                "C" => {
                    create_floor(world, position);
                    create_chest(world, position)
                }
                _ => {}
            }
        }
    }
}
