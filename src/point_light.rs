use crate::{Color, Point};

#[derive(Debug, PartialEq)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }
}
