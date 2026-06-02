use crate::float::float_equals;
use crate::{
    Matrix, impl_display_for_tuple, impl_div_f32_for_tuple, impl_mul_f32_for_tuple,
    impl_neg_for_tuple, impl_sub_for_tuple,
};
use crate::{Tuple, Vector, impl_partial_eq_for_tuple};
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Point {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

impl Tuple for Point {
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
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::new(
            self.x + rhs.x(),
            self.y + rhs.y(),
            self.z + rhs.z(),
            self.w + rhs.w(),
        )
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z(),
            self.w - rhs.w(),
        )
    }
}

impl Mul<&Matrix<4, 4>> for &Point {
    type Output = Point;

    fn mul(self, rhs: &Matrix<4, 4>) -> Self::Output {
        rhs * self
    }
}

impl_display_for_tuple!(Point);
impl_partial_eq_for_tuple!(Point);
impl_sub_for_tuple!(Point);
impl_neg_for_tuple!(Point);
impl_mul_f32_for_tuple!(Point);
impl_div_f32_for_tuple!(Point);
