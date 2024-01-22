use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;

use std::time::Duration;

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
    let texture_created = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

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
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
