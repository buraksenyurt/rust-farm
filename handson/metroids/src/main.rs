mod constants;
mod texture_manager;
mod utility;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::path::Path;
use std::thread::sleep;

use crate::constants::{
    SCREEN_HEIGHT, SCREEN_WIDTH, SPACE_SHIP_HEIGHT, SPACE_SHIP_OUTPUT_HEIGHT,
    SPACE_SHIP_OUTPUT_WIDTH, SPACE_SHIP_WIDTH,
};
use crate::texture_manager::TextureManager;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::{Font, FontStyle};
use sdl2::video::WindowContext;
use std::time::Duration;

fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut TextureManager<WindowContext>,
    _texture_creator: &TextureCreator<WindowContext>,
    _font: &Font,
    key_manager: &HashMap<String, bool>,
) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    let src = Rect::new(0, 0, SPACE_SHIP_WIDTH, SPACE_SHIP_HEIGHT);
    let x = SCREEN_WIDTH / 2;
    let y = SCREEN_HEIGHT / 2;
    let dest = Rect::new(
        (x - (SPACE_SHIP_OUTPUT_WIDTH / 2)) as i32,
        (y - (SPACE_SHIP_OUTPUT_HEIGHT / 2)) as i32,
        SPACE_SHIP_OUTPUT_WIDTH,
        SPACE_SHIP_OUTPUT_HEIGHT,
    );
    let center = Point::new(
        (SPACE_SHIP_OUTPUT_WIDTH / 2) as i32,
        (SPACE_SHIP_OUTPUT_HEIGHT / 2) as i32,
    );
    let texture = texture_manager.load("assets/space_ship.png")?;
    let mut angle = 0.;
    if utility::is_key_pressed(&key_manager, "W") {
        angle = 0.;
    } else if utility::is_key_pressed(&key_manager, "D") {
        angle = 90.;
    } else if utility::is_key_pressed(&key_manager, "S") {
        angle = 180.;
    } else if utility::is_key_pressed(&key_manager, "A") {
        angle = 270.;
    }
    canvas.copy_ex(&texture, src, dest, angle, center, false, false)?;

    // let greetings = "M E T R O I D S".to_string();
    // let surface = font
    //     .render(&greetings)
    //     .blended(Color::RGBA(255, 0, 0, 125))
    //     .map_err(|e| e.to_string())?;
    // let texture = texture_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string())?;
    // let target = Rect::new(300, 200, 200, 100);
    // canvas.copy(&texture, None, Some(target))?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    println!("Starting game");

    let sdl_context = sdl2::init()?;
    let video_subsystems = sdl_context.video()?;
    let windows = video_subsystems
        .window("Metroids", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Video sub system error");
    let mut canvas = windows.into_canvas().build().expect("Canvas build error");

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);
    texture_manager.load("assets/space_ship.png")?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager = HashMap::new();

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
                Event::KeyDown { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utility::key_down(&mut key_manager, key.to_string());
                    }
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utility::key_up(&mut key_manager, key.to_string());
                    }
                },
                _ => {}
            }
        }
        render(
            &mut canvas,
            &mut texture_manager,
            &texture_creator,
            &font,
            &key_manager,
        )?;
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
