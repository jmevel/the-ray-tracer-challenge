use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::ExtendedTuple;
use crate::float::float_equals;
use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct Point{
    x: f32,
    y: f32,
    z: f32,
    w:f32,
}

impl ExtendedTuple for Point{
    fn x(&self) -> &f32{
        &self.x
    }
    fn y(&self) -> &f32{
        &self.y
    }
    fn z(&self) -> &f32{
        &self.z
    }
    fn w(&self) -> &f32{
        &self.w
    }

    fn new(x: f32, y: f32, z: f32, w: Option<f32>) -> Self
    where
        Self: Sized
    {
        Self{x, y, z, w: w.unwrap_or(1.0)}
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::new(
            self.x + rhs.x(),
            self.y + rhs.y(),
            self.z + rhs.z(),
            None
        )
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.x(), &other.x())
            && float_equals(&self.y(), &other.y())
            && float_equals(&self.z(), &other.z())
            && float_equals(&self.w(), &other.w())
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for &Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            None
        )
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z(),
            None
        )
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        &self - &rhs
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z, Some(-self.w))
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Self::Output {
        Point::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            Some(self.w * scalar),
        )
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl Div<f32> for &Point {
    type Output = Point;

    fn div(self, fraction: f32) -> Self::Output {
        ExtendedTuple::new(
            self.x / fraction,
            self.y / fraction,
            self.z / fraction,
            Some(self.w / fraction),
        )
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, fraction: f32) -> Self::Output {
        &self / fraction
    }
}