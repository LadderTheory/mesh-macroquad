use macroquad::prelude::*;

use crate::particle::Particle;
use crate::line::Line;

pub trait Render {
    fn render(&self, color: Color);
}

impl Render for Particle {
    fn render(&self, color: Color) {
        draw_circle(self.position.x as f32, self.position.y as f32, self.radius as f32, color)
    }
}

impl Render for Line {
    fn render(&self, color: Color) {
        draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, 1.0, color)
    }
}