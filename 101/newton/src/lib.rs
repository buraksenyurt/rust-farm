/*
   Kütüphanelerde test modülleri bulunur.
   Örnekte newton kütüphanesi için help dokümantasyonu da hazırlanmıştır.
   Rust'taki tüm kütüphaneler bu tip yardım dokümanlarına sahiptir.
   Yardım dokümanları html çıktısı olarak üretilir ve yazılırken markdown standartları kullanılır

*/
#[cfg(test)]
mod tests {
    use crate::get_average_speed;

    #[test]
    fn should_average_speed_works() {
        // fonksiyon çağrısı
        let result = get_average_speed(560.0, 5.5);
        // assert_eq! makrosu ile kabul kriterinin kontrolü
        assert_eq!(result, 101.818184);
    }
}

/// # Get Average Speed Function
/// Returns the average speed of a vehicle.
///
///! ```
///! fn get_average_speed(distance: f32, duration: f32) -> f32
///! ```
/// ## Arguments
///
/// * `distance` - The total distance traveled by the vehicle.
/// * `duration` - How long did it take to travel that distance?
///
/// ## Sample
///
/// ```
/// use newton::get_average_speed;
///
/// let v=get_average_speed(10.0,1.0);
/// assert_eq!(v,10.0);
/// ```
/// # Errors
///
/// This functions errors ....
///
/// # Panics
///
/// This function panics ....
///
pub fn get_average_speed(distance: f32, duration: f32) -> f32 {
    distance / duration
}
