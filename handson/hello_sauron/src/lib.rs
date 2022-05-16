use sauron::prelude::*;

// WASM web tarayıcısına yüklendikten sonra tetiklenecek fonksiyonu tanımlıyoruz
#[wasm_bindgen(start)]
pub fn run() {
    // Doküman nesnesine neyi bağlayacağımızı belirtiyoruz
    Program::mount_to_body(Quote::new());
}

// Model nesnesi için bir veri yapısı tanımladık
struct Quote {
    text: String,
}

impl Quote {
    fn new() -> Self {
        Self {
            text: String::from("Günün özlü sözü..."),
        }
    }
}

// Sauron çatısından gelen Application isimli trait'i uyguluyoruz.
// Trait davranışını Quote veri yapısı için yeniden yazmaktayız.
// Generic parametrede Action enum türünü kullandık.
// Yeniden yazılan fonksiyonlarımız ise view ve update. Buna göre hem arayüz tarafını(view)
// hemde aksiyon karşılığı yapılacakları yazdığımızı ifade edebiliriz.
impl Application<Action> for Quote {
    // Arayüz tarafındaki etkileşimleri Action enum sabiti ile update fonksiyonuna taşımamız söz konusu
    fn view(&self) -> Node<Action> {
        node!(
            <main>
                <h1>"Minimal example"</h1>
                <div>
                    {text(self.text.clone())}
                </div>
                <div>
                    <input type="button" value="Sonraki" on_click=|_|{Action::Forward} />
                    <input type="button" value="Rastgele" on_click=|_|{Action::Random} />
                    <input type="button" value="Önceki" on_click=|_|{Action::Backward} />
                </div>
            </main>
        )
    }
    // Dolayısıyla yapılan hareket ne ise ona uygun bir güncelleme ile arayüzün kullandığı modeli güncellemek mümkün
    fn update(&mut self, action: Action) -> Cmd<Self, Action>
    where
        Self: Sized + 'static,
    {
        match action {
            Action::Backward => self.text = "Önceki özlü söz".to_string(),
            Action::Forward => self.text = "Sonraki özlü söz".to_string(),
            Action::Random => self.text = "Rastgele bir özlü söz".to_string(),
        }
        Cmd::none()
    }
}

// Bazı action tanımları.
// Sonraki söze geç, öncekine geç veya rastgele bir tane getirin ifadesi gibi.
enum Action {
    Forward,
    Backward,
    Random,
}
