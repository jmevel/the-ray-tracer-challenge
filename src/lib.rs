#![allow(incomplete_features)] // required to enable generic_const_exprs without a warning
#![feature(generic_const_exprs)]

pub mod canvas;
pub mod float;
pub mod matrix;
pub mod tuple;

pub use crate::canvas::Canvas;
pub use crate::matrix::Matrix;
pub use crate::tuple::Tuple;
