use crate::graphic::Vector;
use crate::implementation::RenderEngine;

// Köprünün soyutlama(Abstraction) tarafı
pub trait Shape {
    fn draw(&self);
    fn transform(&mut self, position: Vector);
}

// Concrete implementation of shape
pub struct Rectangle {
    pub location: Vector,
    pub width: f32,
    pub height: f32,
    pub renderer: Box<dyn RenderEngine>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, location: Vector, renderer: Box<dyn RenderEngine>) -> Self {
        Self {
            location,
            width,
            height,
            renderer,
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        self.renderer
            .render_rectangle(self.width, self.height, &self.location);
    }

    fn transform(&mut self, position: Vector) {
        self.location = position;
    }
}

// Concrete implementation of shape
pub struct Circle {
    pub location: Vector,
    pub radius: f32,
    pub renderer: Box<dyn RenderEngine>,
}

impl Circle {
    pub fn new(radius: f32, location: Vector, renderer: Box<dyn RenderEngine>) -> Self {
        Self {
            radius,
            location,
            renderer,
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.renderer.render_circle(self.radius, &self.location);
    }

    fn transform(&mut self, position: Vector) {
        self.location = position;
    }
}
