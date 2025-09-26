use std::ops::{Add, Deref, Div, Mul, Neg, Sub};
use crate::float::float_equals;

impl Add for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn add(self, rhs: Self) -> Self::Output {
        ExtendedTuple::new(
            self.0.0 + rhs.0.0,
            self.0.1 + rhs.0.1,
            self.0.2 + rhs.0.2,
            self.0.3 + rhs.0.3,
        )
    }
}

impl Sub for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn sub(self, rhs: Self) -> Self::Output {
        ExtendedTuple::new(
            self.0.0 - rhs.0.0,
            self.0.1 - rhs.0.1,
            self.0.2 - rhs.0.2,
            self.0.3 - rhs.0.3,
        )
    }
}

impl Neg for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn neg(self) -> Self::Output {
        ExtendedTuple::new(-self.0.0, -self.0.1, -self.0.2, -self.0.3)
    }
}

impl Mul<f32> for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn mul(self, scalar: f32) -> Self::Output {
        ExtendedTuple::new(self.0.0 * scalar, self.0.1 * scalar, self.0.2 * scalar, self.0.3 * scalar)
    }
}

impl Div<f32> for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn div(self, fraction: f32) -> Self::Output {
        ExtendedTuple::new(self.0.0 / fraction, self.0.1 / fraction, self.0.2 / fraction, self.0.3 / fraction)
    }
}

impl PartialEq for ExtendedTuple {
    fn eq(&self, other: &Self) -> bool {
        float_equals(self.0.0, other.0.0)
            && float_equals(self.0.1, other.0.1)
            && float_equals(self.0.2, other.0.2)
            && float_equals(self.0.3, other.0.3)
    }
}

impl Eq for ExtendedTuple {}

#[derive(Debug)]
pub struct ExtendedTuple((f32, f32, f32, f32));

impl ExtendedTuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self((x, y, z, w))
    }
    pub fn new_point(x: f32, y: f32, z: f32) -> ExtendedTuple {
        ExtendedTuple((x, y, z, 1.0))
    }
    pub fn new_vector(x: f32, y: f32, z: f32) -> ExtendedTuple {
        ExtendedTuple((x, y, z, 0.0))
    }
    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
    pub fn magnitude(&self) -> f32 {
        (self.0.0.powi(2) + self.0.1.powi(2) + self.0.2.powi(2) + self.0.3.powi(2)).sqrt()
    }
    pub  fn normalize(&self) -> ExtendedTuple {
        let mag = self.magnitude();
        ExtendedTuple((self.0.0/mag, self.0.1/mag, self.0.2/mag, self.0.3/mag))
    }
}

impl Deref for ExtendedTuple {
    type Target = (f32, f32, f32, f32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


