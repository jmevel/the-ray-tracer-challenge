#![allow(incomplete_features)] // required to enable generic_const_exprs without a warning
#![feature(generic_const_exprs)]

pub mod canvas;
pub mod color;
pub mod computations;
pub mod float;
pub mod intersection;
pub mod intersections;
pub mod material;
pub mod matrix;
pub mod object;
pub mod point;
pub mod point_light;
pub mod ray;
pub mod reflection_value;
pub mod sphere;
pub mod tuple;
pub mod vector;
pub mod world;

pub use crate::canvas::Canvas;
pub use crate::color::Color;
pub use crate::computations::Computations;
pub use crate::intersection::Intersection;
pub use crate::intersections::Intersections;
pub use crate::material::Material;
pub use crate::matrix::Matrix;
pub use crate::object::Object;
pub use crate::point::Point;
pub use crate::point_light::PointLight;
pub use crate::ray::Ray;
pub use crate::reflection_value::ReflectionValue;
pub use crate::sphere::Sphere;
pub use crate::tuple::Tuple;
pub use crate::vector::Vector;
pub use crate::world::World;
