use common::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut circles = vec![];

    let window = video_subsystem
        .window("Mountain Scene", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        canvas.set_draw_color(Color::RGB(107, 140, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("{x}:{y}")
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        circles.push(Circle::new(Point::new(x, y), 5));
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let max_peak = 400;
        let peaks = [
            Point::new(0, HEIGHT - 10),
            Point::new(100, HEIGHT - 10),
            Point::new(200, max_peak),
            Point::new(300, max_peak),
            Point::new(450, 500),
            Point::new(500, max_peak),
            Point::new(620, HEIGHT / 2),
            Point::new(700, max_peak),
            Point::new(800, 400),
        ];
        for i in 0..peaks.len() - 1 {
            //canvas.draw_line(peaks[i], peaks[i + 1])?;
            draw_strong_line(
                &mut canvas,
                peaks[i],
                peaks[i + 1],
                Color::RGB(255, 0, 0),
                5,
            )?;
        }

        for c in &circles {
            c.draw_circle(&mut canvas, Color::RGB(255, 255, 255))?;
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
