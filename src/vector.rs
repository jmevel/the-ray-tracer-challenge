use crate::float::float_equals;
use crate::ExtendedTuple;
use std::ops::{Add, Div, Mul, Neg};

#[derive(Debug, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0f32 {
            panic!("Magnitude is 0");
        }
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
    pub fn dot_product(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross_product(&self, other: &Vector) -> Vector {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            None,
        )
    }
}

impl ExtendedTuple for Vector {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }

    fn new(x: f32, y: f32, z: f32, w: Option<f32>) -> Self
    where
        Self: Sized,
    {
        Self {
            x,
            y,
            z,
            w: w.unwrap_or(0.0),
        }
    }
}

impl Eq for Vector {}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.x(), &other.x())
            && float_equals(&self.y(), &other.y())
            && float_equals(&self.z(), &other.z())
            && float_equals(&self.w(), &other.w())
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, None)
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z, Some(-self.w))
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            Some(self.w * scalar),
        )
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, fraction: f32) -> Self::Output {
        Vector::new(
            self.x / fraction,
            self.y / fraction,
            self.z / fraction,
            Some(self.w / fraction),
        )
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, fraction: f32) -> Self::Output {
        &self / fraction
    }
}
