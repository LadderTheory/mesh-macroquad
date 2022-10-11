use std::f32::consts::E;

use rand::Rng;

use crate::math::{Vector2D};

enum Border {
    North,
    East,
    South,
    West,
}

use Border::*;

#[derive(Clone)]
pub struct Particle {
    pub position: Vector2D,
    velocity: Vector2D,
    pub radius: f64,
    pub field_size: Vector2D,
}

impl Particle {
    pub fn new_random(field_size: Vector2D) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            position: Vector2D::new(rng.gen::<f32>() * field_size.x, rng.gen::<f32>() * field_size.y),
            //position: Vector2D { x: 100.0, y: 100.0 },
            velocity: Vector2D::new(1.0 - rng.gen::<f32>() * 2.0, 1.0 - rng.gen::<f32>() * 2.0),
            radius: 2.0,
            field_size,
        }
    }

    pub fn new(position: Vector2D, velocity: Vector2D, radius: f64, field_size: Vector2D) -> Particle {
        Particle {
            position,
            velocity,
            radius,
            field_size,
        }
    }

    fn teleport(&mut self, border: Border) {
        match border {
            North => self.position.y = self.field_size.y,
            West => self.position.x = self.field_size.x,
            South => self.position.y = 0.0,
            East => self.position.x = 0.0,
        }
    }

    fn bounce(&mut self, border: Border) {
        match border {
            North | South => self.velocity.y *= -1.0,
            East | West => self.velocity.x *= -1.0,
        }

        match border {
            North => self.position.y = 0.0,
            East => self.position.x = self.field_size.x,
            South => self.position.y = self.field_size.y,
            West => self.position.x = 0.0,
        }
    }

    fn border(&mut self) {
        let border = if self.position.y < 0.0 {
            Some(North)
        } else if self.position.x < 0.0 {
            Some(West)
        } else if self.position.y > self.field_size.y {
            Some(South)
        } else if self.position.x > self.field_size.x {
            Some(East)
        } else {
            None
        };

        if let Some(s) = border {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.0) {
                self.teleport(s);
            } else {
                self.bounce(s);
            }
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.border();
    }
}