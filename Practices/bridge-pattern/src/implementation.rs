use crate::graphic::Vector;

// Köprünün uyarlama(Implementation) tarafındaki tanım
pub trait RenderEngine {
    fn render_rectangle(&self, width: f32, height: f32, position: &Vector);
    fn render_circle(&self, radius: f32, position: &Vector);
}

// Concrete implementation
#[derive(Copy, Clone)]
pub struct DirectX;
impl RenderEngine for DirectX {
    fn render_rectangle(&self, width: f32, height: f32, position: &Vector) {
        println!("DirectX ile {position} koordinatlarında {width}X{height} boyutlarında dörtgen çizilir.");
    }

    fn render_circle(&self, radius: f32, position: &Vector) {
        println!("DirectX ile {position} koordinatlarında {radius} çapında daire çizilir.");
    }
}

// Concrete implementation
#[derive(Copy, Clone)]
pub struct OpenGl;
impl RenderEngine for OpenGl {
    fn render_rectangle(&self, width: f32, height: f32, position: &Vector) {
        println!(
            "OpenGL ile {position} koordinatlarında {width}X{height} boyutlarında dörtgen çizilir."
        );
    }

    fn render_circle(&self, radius: f32, position: &Vector) {
        println!("OpenGL ile {position} koordinatlarında {radius} çapında daire çizilir.");
    }
}
