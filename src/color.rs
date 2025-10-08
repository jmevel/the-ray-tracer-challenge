use std::ops::Mul;
use crate::float::float_equals;
use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Color{
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn red(&self) -> &f32 { &self.red }
    pub fn green(&self) -> &f32 { &self.green }
    pub fn blue(&self) -> &f32 { &self.blue }
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

impl Eq for Color {}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.red(), &other.red())
           && float_equals(&self.green(), &other.green())
        && float_equals(&self.blue(), &other.blue())
        
    }
}

impl Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Self::Output {
        Color::new(
            self.red * scalar,
            self.green * scalar,
            self.blue * scalar,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        &self * other
    }
}