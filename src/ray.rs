use crate::{Matrix, Tuple};

#[derive(Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        if !origin.is_point() {
            panic!("origin must be a point");
        }
        if !direction.is_vector() {
            panic!("direction must be a vector");
        }

        Self { origin, direction }
    }

    pub fn origin(&self) -> &Tuple {
        &self.origin
    }
    pub fn direction(&self) -> &Tuple {
        &self.direction
    }

    pub fn position(&self, t: f32) -> Tuple {
        &self.origin + &(&self.direction * t)
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
