use crate::float::float_equals;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct ExtendedTuple
{
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl ExtendedTuple {
    pub fn x(&self) -> &f32{
        &self.x
    }
    pub fn y(&self) -> &f32{
        &self.y
    }
    pub fn z(&self) -> &f32{
        &self.z
    }
    pub fn w(&self) -> &f32{
        &self.w
    }
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn new_point(x: f32, y: f32, z: f32) -> ExtendedTuple {
        ExtendedTuple { x, y, z, w: 1.0 }
    }
    pub fn new_vector(x: f32, y: f32, z: f32) -> ExtendedTuple {
        ExtendedTuple { x, y, z, w: 0.0 }
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> ExtendedTuple {
        if !self.is_vector() {
            panic!("Only a vector can be normalized");
        }
        let mag = self.magnitude();
        if mag == 0f32 {
            panic!("Magnitude is 0");
        }
        ExtendedTuple { x: self.x / mag, y: self.y / mag, z: self.z / mag, w: self.w / mag }
    }
    pub fn dot_product(&self, other: &ExtendedTuple) -> f32 {
        if !self.is_vector() || !other.is_vector() {
            panic!("Dot product can only be applied on vectors");
        }
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross_product(&self, other: &ExtendedTuple) -> ExtendedTuple {
        if !self.is_vector() || !other.is_vector() {
            panic!("Dot product can only be applied on vectors");
        }
        ExtendedTuple::new_vector(self.y * other.z - self.z * other.y,
                                  self.z * other.x - self.x * other.z,
                                  self.x * other.y - self.y * other.x)
    }
}

impl Eq for ExtendedTuple {}

impl PartialEq for ExtendedTuple {
    fn eq(&self, other: &Self) -> bool {
        float_equals(self.x, other.x)
            && float_equals(self.y, other.y)
            && float_equals(self.z, other.z)
            && float_equals(self.w, other.w)
    }
}

impl Add for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn add(self, rhs: Self) -> Self::Output {
        ExtendedTuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn sub(self, rhs: Self) -> Self::Output {
        ExtendedTuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn neg(self) -> Self::Output {
        ExtendedTuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn mul(self, scalar: f32) -> Self::Output {
        ExtendedTuple::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl Div<f32> for &ExtendedTuple {
    type Output = ExtendedTuple;

    fn div(self, fraction: f32) -> Self::Output {
        ExtendedTuple::new(self.x / fraction, self.y / fraction, self.z / fraction, self.w / fraction)
    }
}



