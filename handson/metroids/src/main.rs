use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::thread::sleep;

use sdl2::ttf::{Font, FontStyle};
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

fn render(canvas:&mut WindowCanvas,texture_creator: &TextureCreator<WindowContext>,font:&Font)->Result<(),String>{
    let color=Color::RGB(0,0,0);
    canvas.set_draw_color(color);
    canvas.clear();

    let greetings="M E T R O I D S".to_string();
    let surface=font.render(&greetings).blended(Color::RGBA(255,0,0,125)).map_err(|e|e.to_string())?;
    let texture =texture_creator.create_texture_from_surface(&surface).map_err(|e|e.to_string())?;
    let target=Rect::new(300,200,200,100);
    canvas.copy(&texture,None,Some(target))?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    println!("Starting game");

    let sdl_context = sdl2::init()?;
    let video_subsystems = sdl_context.video()?;
    let windows = video_subsystems
        .window("Metroids", 800, 600)
        .position_centered()
        .build()
        .expect("Video sub system error");
    let mut canvas = windows.into_canvas().build().expect("Canvas build error");
    let texture_creater = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'run;
                }
                _ => {}
            }
        }
        render(&mut canvas,&texture_creater,&font);
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
