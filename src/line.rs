use crate::math::{Vector2D};

#[derive(Clone)]
pub struct Line {
    pub p1: Vector2D,
    pub p2: Vector2D,
}

impl Line {
    pub fn new(p1: Vector2D, p2: Vector2D) -> Self {
        Self {
            p1,
            p2,
        }
    }

    pub fn get_points(&self) -> (Vector2D, Vector2D) {
        (self.p1, self.p2)
    }

    pub fn len(&self) -> f32 {
        Vector2D::distance(self.p1, self.p2)
    }

    pub fn middle(&self) -> Vector2D {
        self.sum() / Vector2D::new(2.0, 2.0)
    }

    pub fn sum(&self) -> Vector2D {
        self.p1 + self.p2
    }
}