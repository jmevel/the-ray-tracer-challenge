use std::any::Any;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TupleType {
    Point,
    Vector,
}

pub trait ExtendedTuple {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
    fn new(x: f32, y: f32, z: f32, w: Option<f32>) -> Self
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> TupleType;
}

impl Debug for dyn ExtendedTuple {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ExtendedTuple{{{}}}", self.x())
    }
}

// impl ExtendedTuple {
//     pub fn new_point(x: f32, y: f32, z: f32) -> ExtendedTuple {
//         ExtendedTuple { x, y, z, w: 1.0 }
//     }
//     pub fn new_color(red: f32, green: f32, blue: f32) -> ExtendedTuple {
//         Self::new_point(red, green, blue)
//     }
//     pub fn new_vector(x: f32, y: f32, z: f32) -> ExtendedTuple {
//         ExtendedTuple { x, y, z, w: 0.0 }
//     }
//     pub fn is_point(&self) -> bool {
//         self.w == 1.0
//     }
//     pub fn is_vector(&self) -> bool {
//         self.w == 0.0
//     }
//     pub fn is_color(&self) -> bool {
//         self.is_point()
//     }
//     pub fn magnitude(&self) -> f32 {
//         if !self.is_vector() {
//             panic!("Magnitude only makes sense on vectors")
//         }
//         (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
//     }
//     pub fn normalize(&self) -> ExtendedTuple {
//         if !self.is_vector() {
//             panic!("Only a vector can be normalized");
//         }
//         let mag = self.magnitude();
//         if mag == 0f32 {
//             panic!("Magnitude is 0");
//         }
//         ExtendedTuple {
//             x: self.x / mag,
//             y: self.y / mag,
//             z: self.z / mag,
//             w: self.w / mag,
//         }
//     }
//     pub fn dot_product(&self, other: &ExtendedTuple) -> f32 {
//         if !self.is_vector() || !other.is_vector() {
//             panic!("Dot product can only be applied on vectors");
//         }
//         self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
//     }
//     pub fn cross_product(&self, other: &ExtendedTuple) -> ExtendedTuple {
//         if !self.is_vector() || !other.is_vector() {
//             panic!("Dot product can only be applied on vectors");
//         }
//         ExtendedTuple::new_vector(
//             self.y * other.z - self.z * other.y,
//             self.z * other.x - self.x * other.z,
//             self.x * other.y - self.y * other.x,
//         )
//     }
// }

// impl Mul<&ExtendedTuple> for &ExtendedTuple {
//     type Output = ExtendedTuple;
//
//     fn mul(self, other: &ExtendedTuple) -> Self::Output {
//         ExtendedTuple::new(
//             self.x * other.x,
//             self.y * other.y,
//             self.z * other.z,
//             self.w * other.w,
//         )
//     }
// }
//
// impl Mul<&ExtendedTuple> for ExtendedTuple {
//     type Output = ExtendedTuple;
//
//     fn mul(self, other: &ExtendedTuple) -> Self::Output {
//         &self * other
//     }
// }
