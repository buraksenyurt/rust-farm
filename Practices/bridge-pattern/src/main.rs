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
mod abstraction;
mod graphic;
mod implementation;

use crate::abstraction::*;
use crate::graphic::*;
use crate::implementation::*;

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
