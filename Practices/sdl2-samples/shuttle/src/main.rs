use common::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut velocity = Vector::new(0., 0.);
    let mut shuttle = Shuttle::new(Point::new(200, 80), 100);

    let window = video_subsystem
        .window(
            format!("Shuttle Scene. Fuel {}", shuttle.fuel_level).as_str(),
            WIDTH as u32,
            HEIGHT as u32,
        )
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
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    velocity.x -= 0.25;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    velocity.x += 0.25;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    velocity.y -= 1.;
                    shuttle.fuel_level -= 1;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        draw_mountain(&mut canvas)?;
        shuttle.draw(&mut canvas, Color::RGB(255, 255, 0), velocity.to_point())?;
        velocity.y += 0.05;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_mountain(canvas: &mut WindowCanvas) -> Result<(), String> {
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
        draw_strong_line(canvas, peaks[i], peaks[i + 1], Color::RGB(255, 255, 255), 5)?;
    }
    Ok(())
}

pub struct Shuttle {
    pub top_left: Point,
    pub fuel_level: i32,
}

impl Shuttle {
    pub fn new(top_left: Point, fuel_level: i32) -> Self {
        Self {
            top_left,
            fuel_level,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        velocity: Point,
    ) -> Result<(), String> {
        let point = self.top_left;

        canvas.set_draw_color(color);
        canvas.draw_rect(Rect::new(
            point.x + velocity.x,
            point.y + velocity.y,
            25,
            25,
        ))?;
        canvas.draw_line(
            Point::new(point.x + velocity.x, point.y + 25 + velocity.y),
            Point::new(point.x - 10 + velocity.x, point.y + 55 + velocity.y),
        )?;
        canvas.draw_line(
            Point::new(point.x + 25 + velocity.x, point.y + 25 + velocity.y),
            Point::new(point.x + 35 + velocity.x, point.y + 55 + velocity.y),
        )?;

        Ok(())
    }
}
