use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let video = context.video()?;

    let window = video
        .window("Hello SDL2", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_handler = context.event_pump()?;
    'running: loop {
        for event in event_handler.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        println!("Lef mouse button clicked -> {x}:{y}")
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("{x}:{y}")
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(107, 140, 255));
        canvas.clear();

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
