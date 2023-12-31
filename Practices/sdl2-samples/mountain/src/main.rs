use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Mountain Scene", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    canvas.set_draw_color(Color::RGB(107, 140, 255));
    canvas.clear();

    'running: loop {

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
                        draw_circle(&mut canvas, Point::new(x, y), 5, Color::RGB(255, 255, 255))?;
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let max_peak = 400;
        let peaks = [
            (0, HEIGHT - 10),
            (100, HEIGHT - 10),
            (200, max_peak),
            (300, max_peak),
            (450, 500),
            (500, max_peak),
            (620, HEIGHT / 2),
            (700, max_peak),
            (800, 400),
        ];
        for i in 0..peaks.len() - 1 {
            canvas.draw_line(peaks[i], peaks[i + 1])?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_circle(
    canvas: &mut Canvas<Window>,
    center: Point,
    radius: i32,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..360 {
        let radian = (i as f64 * std::f64::consts::PI) / 180.0;
        let x = center.x() + (radian.cos() * radius as f64) as i32;
        let y = center.y() + (radian.sin() * radius as f64) as i32;
        canvas.draw_point(Point::new(x, y))?;
    }
    Ok(())
}
