use crate::{Matrix, Point, Vector};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }
    pub fn direction(&self) -> &Vector {
        &self.direction
    }

    pub fn position(&self, t: &f32) -> Point {
        &self.origin + &(&self.direction * *t)
    }

    pub fn transform(&self, transformation: &Matrix<4, 4>) -> Self {
        let new_origin = self.origin() * transformation;
        let new_direction = self.direction() * transformation;

        Self {
            origin: new_origin,
            direction: new_direction,
        }
    }
}
