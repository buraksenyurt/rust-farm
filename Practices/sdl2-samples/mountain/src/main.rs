use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;
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
                        circles.push(Circle {
                            center: Point::new(x, y),
                            radius: 5,
                        });
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
            draw_circle(&mut canvas, c, Color::RGB(255, 255, 255))?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

#[derive(Copy, Clone)]
struct Circle {
    pub center: Point,
    pub radius: i32,
}

fn draw_circle(canvas: &mut Canvas<Window>, circle: &Circle, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..360 {
        let radian = (i as f64 * PI) / 180.0;
        let x = circle.center.x + (radian.cos() * circle.radius as f64) as i32;
        let y = circle.center.y + (radian.sin() * circle.radius as f64) as i32;
        canvas.draw_point(Point::new(x, y))?;
    }
    Ok(())
}

fn draw_strong_line(
    canvas: &mut Canvas<Window>,
    start: Point,
    end: Point,
    color: Color,
    thickness: i32,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..thickness {
        let offset = i - thickness / 2;
        canvas.draw_line(
            Point::new(start.x + offset, start.y + offset),
            Point::new(end.x + offset, end.y + offset),
        )?;
    }
    Ok(())
}
