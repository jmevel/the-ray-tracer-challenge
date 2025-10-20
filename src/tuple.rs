use crate::float::float_equals;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TupleType {
    Point,
    Vector,
    Color,
}

#[derive(Debug, Clone)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    tuple_type: TupleType,
}

impl Tuple {
    pub fn x(&self) -> &f32 {
        &self.x
    }
    pub fn y(&self) -> &f32 {
        &self.y
    }
    pub fn z(&self) -> &f32 {
        &self.z
    }
    pub fn w(&self) -> &f32 {
        &self.w
    }
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let tuple_type = match w {
            1f32 => TupleType::Point,
            0f32 | _ => TupleType::Vector,
        };
        Self {
            x,
            y,
            z,
            w,
            tuple_type,
        }
    }
    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 1.0,
            tuple_type: TupleType::Point,
        }
    }
    pub fn new_color(red: f32, green: f32, blue: f32) -> Tuple {
        Tuple {
            x: red,
            y: green,
            z: blue,
            w: 1.0,
            tuple_type: TupleType::Color,
        }
    }
    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 0.0,
            tuple_type: TupleType::Vector,
        }
    }
    pub fn is_point(&self) -> bool {
        self.tuple_type == TupleType::Point
    }
    pub fn is_vector(&self) -> bool {
        self.tuple_type == TupleType::Vector
    }
    pub fn is_color(&self) -> bool {
        self.tuple_type == TupleType::Color
    }
    pub fn magnitude(&self) -> f32 {
        if !self.is_vector() {
            panic!("Magnitude only makes sense on vectors")
        }
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        if !self.is_vector() {
            panic!("Only a vector can be normalized");
        }
        let mag = self.magnitude();
        if mag == 0f32 {
            panic!("Magnitude is 0");
        }
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
            tuple_type: TupleType::Vector,
        }
    }
    pub fn dot_product(&self, other: &Tuple) -> f32 {
        if !self.is_vector() || !other.is_vector() {
            panic!("Dot product can only be applied on vectors");
        }
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross_product(&self, other: &Tuple) -> Tuple {
        if !self.is_vector() || !other.is_vector() {
            panic!("Dot product can only be applied on vectors");
        }
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Eq for Tuple {}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.x, &other.x)
            && float_equals(&self.y, &other.y)
            && float_equals(&self.z, &other.z)
            && float_equals(&self.w, &other.w)
    }
}

impl Add for &Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Mul<f32> for &Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f32) -> Self::Output {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl Mul<&Tuple> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Self::Output {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl Mul<&Tuple> for Tuple {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Self::Output {
        &self * other
    }
}

impl Div<f32> for &Tuple {
    type Output = Tuple;

    fn div(self, fraction: f32) -> Self::Output {
        Tuple::new(
            self.x / fraction,
            self.y / fraction,
            self.z / fraction,
            self.w / fraction,
        )
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, fraction: f32) -> Self::Output {
        &self / fraction
    }
}
