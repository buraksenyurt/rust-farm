use crate::prelude::*;

// Render sistemimiz. Oyun context'i ile çalışır.
// Render sisteminin kendisine dahil olan referansların ne kadar süre yaşayacağını kestirmesi
// zor olduğundan açıkça lifetime bildirim kulllanılmakta
pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

// Rendering veri yapısının bir sistem olabilmesi için System trait'ini uygulaması gerekiyor.
impl<'a> System<'a> for RenderingSystem<'a> {
    // Render sistemine dahil olacak veri depoları.
    // Şu an için Position ve Renderable bileşenlerinin kullanıldığı nesneler ele alınıyor
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    // nesnelerin render edilmesi işinin üstlenildiği fonksiyon
    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
        // ekranı bir arka plan rengi kullanarak temizle
        clear(self.context, Color::new(0.95, 0.95, 0.95, 1.0));
        // Renderable bileşeninin kullanan varlıkları bir toplamamız lazım
        let mut entities = (&positions, &renderables).join().collect::<Vec<_>>();
        // z değerine göre bir sıralama yapmaktayız.
        // Bu hangi entity katmanının üstte kalacağını belirlemek için bir yol
        entities.sort_by_key(|&k| k.0.z);

        // entity nesnelerini dolaşıyoruz
        for (p, r) in entities.iter() {
            // her birisi için resmi oluşturuyoruz.
            let image =
                Image::new(self.context, r.asset_path.clone()).expect("image does not exist error");
            // resmin x,y değerlerini hesaplarken 32 pixel büyüklüğü ifade eden TILE_WIDTH sabiti kullanılıyor.
            let (x, y) = (p.x as f32 * TILE_WIDTH, p.y as f32 * TILE_WIDTH);

            // imajın çizileceği yeri hazırlayıp
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            // context nesnesini kullanan draw fonksiyonunua vererek ekrana bastırıyoruz.
            draw(self.context, &image, draw_params).expect("drawing operations error");
        }

        // hazırlanan context nesnesini sunuş moduna alıyoruz.
        present(self.context).expect("presentation error");
    }
}
