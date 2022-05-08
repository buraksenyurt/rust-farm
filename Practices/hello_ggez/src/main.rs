use ggez::conf::{Conf, WindowMode};
use ggez::event::{run, EventHandler};
use ggez::{graphics, mint, timer, Context, ContextBuilder, GameError};
use std::time::Duration;

/*
   Her oyunun bir döngüsü vardır(Game Loop olarak ifade edilir)
   Oyun döngüsünde klavye, mouse, pencere kapatma gibi olaylar, oyuncunun pozisyonu, puanı
   gibi durum verileri, şekil ve imgelerin ekrana çizilmesi gibi işler ele alınır.

   ggez küfesinde bu iş için EventHandler isimli trait kullanılır.
*/

fn main() {
    let mut shapes = Vec::new();
    shapes.push(Shape::Circle(mint::Point2 { x: 200.0, y: 150.0 }, 25.0));
    shapes.push(Shape::Circle(mint::Point2 { x: 250.0, y: 100.0 }, 25.0));
    shapes.push(Shape::Rectangle(graphics::Rect::new(
        290.0, 130.0, 80.0, 10.0,
    )));
    shapes.push(Shape::Circle(mint::Point2 { x: 50.0, y: 200.0 }, 25.0));
    shapes.push(Shape::Rectangle(graphics::Rect::new(
        10.0, 20.0, 100.0, 10.0,
    )));

    // State nesnesi
    //let state = State::new();
    let state = State {
        game_shapes: shapes,
    };

    // Olası konfigurasyon ayarları
    let mut c = Conf::new();
    c.window_setup.title = "First Stage Game".to_string();
    c.window_mode.width = 400.0;
    c.window_mode.height = 400.0;

    // Her oyunun bir Context ve EventLoop nesnesine ihtiyacı olur
    let (ctx, event_loop) = ContextBuilder::new("ggez_first_sample", "burak selim")
        .default_conf(c)
        .build()
        .unwrap();
    // Oyunu Context, EventLoop ve State nesneleri ile başlattığımız kısım
    run(ctx, event_loop, state);
}

// Oyunun o anki durumuna ait tüm veri ve bilgiler bir State nesnesinde tutulur
// Oyuncu pozisyonları, skorlar, oyun sahasındaki diğer varlıklar vb
// Context enesi ile mouse, klavye, zamanlayıcılar, grafik ve ses gibi donanımlara erişebiliriz.
struct State {
    //duration: Duration,
    game_shapes: Vec<Shape>,
}

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

// impl State {
//     pub fn new() -> Self {
//         Self {
//             duration: Duration::new(0, 0),
//         }
//     }
// }

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        //self.duration = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::BLACK);
        for shape in &self.game_shapes {
            let obj = match shape {
                &Shape::Rectangle(r) => graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    r,
                    graphics::Color::MAGENTA,
                )?,
                &Shape::Circle(c, radius) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    c,
                    radius,
                    0.1,
                    graphics::Color::YELLOW,
                )?,
            };
            graphics::draw(ctx, &obj, graphics::DrawParam::default())?;
        }
        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     mint::Point2 { x: 100.0, y: 20.0 },
        //     25.0,
        //     0.1,
        //     graphics::Color::MAGENTA,
        // )?;
        // graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        //
        // let rectangle = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     graphics::Rect::new(100.0, 50.0, 100.0, 20.0),
        //     graphics::Color::YELLOW,
        // )?;
        // graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        //println!("Duration ={} nanosaniye", self.duration.as_nanos());
        Ok(())
    }
}
