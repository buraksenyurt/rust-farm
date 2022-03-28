use crate::prelude::*;

// Oyuna bir kamera ekleniyor. Bu kamera ile oyun pencersine gerçek dünyadan bakıp oyuncuya odaklanıyoruz.
// Bu sebeple kamera vizörüne girecek alanın x,y koordinatlarını tutan bir veri yapısı gerekiyor
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    // Kameranın baktığı alanın oyuncunun o an bulunduğu x,y pozisyonuna göre ayarlanması lazım
    // Hesaplamayı yaparken DISPLAY_WIDTH ve DISPLAY_HEIGHT alanları baz alınarak bir dörtgen alanı belirleniyor
    pub fn new(adventurer_location: Point) -> Self {
        Self {
            left_x: adventurer_location.x - DISPLAY_WIDTH / 2,
            right_x: adventurer_location.x + DISPLAY_WIDTH / 2,
            top_y: adventurer_location.y - DISPLAY_HEIGHT / 2,
            bottom_y: adventurer_location.y + DISPLAY_HEIGHT / 2,
        }
    }

    // Oyuncu hareket ettikçe kamera açısının yani oyun penceresinde gördüğü alanın güncellenmesi lazım.
    // Hani oyuncu karakterinin hareket ettirdiğinde oyun sahası da onunla birlikte ilerler ya...
    pub fn on_move(&mut self, adventurer_location: Point) {
        self.left_x = adventurer_location.x - DISPLAY_WIDTH / 2;
        self.right_x = adventurer_location.x + DISPLAY_WIDTH / 2;
        self.top_y = adventurer_location.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = adventurer_location.y + DISPLAY_HEIGHT / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_camera_move_after_player_move() {
        let mut location = Point::new(10, 10);
        let mut camera = Camera::new(location);
        assert_eq!(camera.left_x, -10);
        assert_eq!(camera.right_x, 30);
        assert_eq!(camera.top_y, -2);
        assert_eq!(camera.bottom_y, 22);

        location.x = 12;
        location.y = 12;
        camera.on_move(location);
        assert_eq!(camera.left_x, -8);
        assert_eq!(camera.right_x, 32);
        assert_eq!(camera.top_y, 0);
        assert_eq!(camera.bottom_y, 24);
    }
}
