use crate::float::float_equals;
use crate::impl_add_for_tuple;
use crate::impl_display_for_tuple;
use crate::impl_div_f32_for_tuple;
use crate::impl_mul_f32_for_tuple;
use crate::impl_mul_for_tuple;
use crate::impl_neg_for_tuple;
use crate::impl_partial_eq_for_tuple;
use crate::impl_sub_for_tuple;
use crate::{Tuple, Vector};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Color {
    pub fn new_color(red: f32, green: f32, blue: f32) -> Color {
        Color {
            x: red,
            y: green,
            z: blue,
            w: 1.0,
        }
    }
}

impl Tuple for Color {
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

impl_display_for_tuple!(Color);
impl_partial_eq_for_tuple!(Color);
impl_add_for_tuple!(Color);
impl_sub_for_tuple!(Color);
impl_neg_for_tuple!(Color);
impl_mul_f32_for_tuple!(Color);
impl_mul_for_tuple!(Color);
impl_div_f32_for_tuple!(Color);
