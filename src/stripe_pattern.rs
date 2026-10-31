use crate::{Color, Point};

#[derive(Debug)]
pub struct StripePattern {
    color_1: Color,
    color_2: Color,
}

impl StripePattern {
    pub fn new(color_1: Color, color_2: Color) -> Self {
        Self { color_1, color_2 }
    }

    pub fn color_1(&self) -> &Color {
        &self.color_1
    }
    pub fn color_2(&self) -> &Color {
        &self.color_2
    }

    pub fn stripe_at(&self, point: &Point) -> &Color {
        if point.x.floor() % 2.0 == 0.0 {
            return self.color_1();
        }
        self.color_2()
    }
}
