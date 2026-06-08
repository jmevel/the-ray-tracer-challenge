use crate::Matrix;
use crate::Tuple;
use crate::float::float_equals;
use crate::impl_add_for_tuple;
use crate::impl_display_for_tuple;
use crate::impl_div_f32_for_tuple;
use crate::impl_mul_f32_for_tuple;
use crate::impl_mul_matrix_for_tuple;
use crate::impl_neg_for_tuple;
use crate::impl_partial_eq_for_tuple;
use crate::impl_sub_for_tuple;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector {
    pub fn new_vector(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn with_w(mut self, w: f32) -> Self {
        self.w = w;
        self
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
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
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        let dot_product = self.dot_product(normal);
        self - &(normal * 2f32 * dot_product)
    }
}

impl Tuple for Vector {
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

impl_display_for_tuple!(Vector);
impl_partial_eq_for_tuple!(Vector);
impl_add_for_tuple!(Vector);
impl_sub_for_tuple!(Vector);
impl_neg_for_tuple!(Vector);
impl_mul_f32_for_tuple!(Vector);
impl_mul_matrix_for_tuple!(Vector);
impl_div_f32_for_tuple!(Vector);
