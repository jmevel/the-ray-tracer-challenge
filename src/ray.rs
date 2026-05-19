use crate::Tuple;

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
}
