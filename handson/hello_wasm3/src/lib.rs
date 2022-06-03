use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_temperature(city: &str) -> String {
    let mut rng = rand::thread_rng();
    let temperature = rng.gen_range(-10..50);
    // alert esasında JS'in bir fonksiyonudur.
    // Dolayısıyla buradaki çağrı sonrası tarayıca alert mesajı işletilecektir.
    alert("Volaaa");
    format!("{} için sıcaklık {} derece", city, temperature)
}

// Dilersek aşağıdaki gibi Javascript tarafında var olan bir fonksiyonun
// rust tarafından çağırılması sağlayabiliriz.
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
