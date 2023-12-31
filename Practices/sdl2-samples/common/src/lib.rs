use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;

pub fn draw_strong_line(
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

pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn draw_circle(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        canvas.set_draw_color(color);
        for i in 0..360 {
            let radian = (i as f64 * PI) / 180.0;
            let x = self.center.x + (radian.cos() * self.radius as f64) as i32;
            let y = self.center.y + (radian.sin() * self.radius as f64) as i32;
            canvas.draw_point(Point::new(x, y))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     // let result = add(2, 2);
    //     // assert_eq!(result, 4);
    // }
}
