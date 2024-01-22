/*
   Örnekte yapısal tasarım kalıplarından bridge deseni ele alınmakta.
   Senaryoda grafiksel şekillerin çizimlerine ait soyutlamalar ile
   farklı render motorlarının implementasyonu arasında köprü kuruluyor.

   Şöyle ki,

   Dörtgen, daire, paralel kenar, üçgen gibi geometrik şekillleri ele alalım.
   Bu şekillerin ekrana çizdirilmesi için farklı renderlar olsun. OpenGL, DirectX gibi.
   Bu render motorları için ortak davranışların şekillerin render edilmesi ki bunu kendi bildikleri şekilde yapıyorlar.

    Dolayısıyla önce RenderEngine trait'ini tanımlayabiliriz. Bu, köprünün implementasyon tarafıdır.
    Yani render operasyonlarının işletildiği yer.

    Render edilecek nesnelerimiz ise geometrik şekiller. Bu nesnelerin draw operasyonlarının render motoruna
    göre yapılmasını istiyoruz. Şekillerin draw fonksiyonelliklerini de mutlaka uyarlaması lazım.
    Bunun için Shape isimli trait'ten yararlanıyoruz.

    Şekilleri temsil eden veri yapıları RenderEngine'leri özellik olarak taşıyorlar. Bu özellik değerleri
    yani şeklin hangi render motorunu kullanacağını new metotlarında belirliyoruz. Bir nevi constructor üstünden
    bağımlı bileşen enjekte ettiğimizi düşünebilirsiniz.

    Bir şekil nesnesi örneklerken new metodu render motorunu da alır. Böylece draw fonksiyonu hangi render
    motoru ile çizim yaptıracağını bilir.

*/
use std::fmt::{Display, Formatter};

fn main() {
    let direct_x = Box::new(DirectX {});
    let open_gl = Box::new(OpenGl {});

    let mut blue_rect = Rectangle::new(10., 20., Vector::new(0., 10., 0.), open_gl.clone());
    blue_rect.draw();

    blue_rect.renderer = direct_x;
    blue_rect.draw();

    let red_circle = Circle::new(3., Vector::new(10., 10., -10.), open_gl);
    red_circle.draw();
}

struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.x, self.y, self.z)
    }
}

// Köprünün uyarlama(Implementation) tarafı
trait RenderEngine {
    fn render_rectangle(&self, width: f32, height: f32, position: &Vector);
    fn render_circle(&self, radius: f32, position: &Vector);
}

#[derive(Copy, Clone)]
struct DirectX;
impl RenderEngine for DirectX {
    fn render_rectangle(&self, width: f32, height: f32, position: &Vector) {
        println!("DirectX ile {position} koordinatlarında {width}X{height} boyutlarında dörtgen çizilir.");
    }

    fn render_circle(&self, radius: f32, position: &Vector) {
        println!("DirectX ile {position} koordinatlarında {radius} çapında daire çizilir.");
    }
}

#[derive(Copy, Clone)]
struct OpenGl;
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

// Köprünün soyutlama(Abstraction) tarafı
trait Shape {
    fn draw(&self);
    fn transform(&mut self, position: Vector);
}

struct Rectangle {
    pub location: Vector,
    pub width: f32,
    pub height: f32,
    pub renderer: Box<dyn RenderEngine>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, location: Vector, renderer: Box<dyn RenderEngine>) -> Self {
        Self {
            location,
            width,
            height,
            renderer,
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        self.renderer
            .render_rectangle(self.width, self.height, &self.location);
    }

    fn transform(&mut self, position: Vector) {
        self.location = position;
    }
}

struct Circle {
    pub location: Vector,
    pub radius: f32,
    pub renderer: Box<dyn RenderEngine>,
}

impl Circle {
    pub fn new(radius: f32, location: Vector, renderer: Box<dyn RenderEngine>) -> Self {
        Self {
            radius,
            location,
            renderer,
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.renderer.render_circle(self.radius, &self.location);
    }

    fn transform(&mut self, position: Vector) {
        self.location = position;
    }
}
