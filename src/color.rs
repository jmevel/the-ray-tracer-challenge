use crate::Material;
use crate::Point;
use crate::PointLight;
use crate::float::float_equals;
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

#[derive(Debug, Clone, Copy, Default)]
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

    pub fn white() -> Color {
        Color::new_color(1f32, 1f32, 1f32)
    }

    pub fn black() -> Color {
        Color::default()
    }

    pub fn red() -> Color {
        Color::new_color(1f32, 0f32, 0f32)
    }

    pub fn lighting(
        material: &Material,
        light: &PointLight,
        position: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
    ) -> Color {
        // combine the surface's color with the light's color/intensity
        let effective_color = &material.color * light.intensity();

        // find the direction to the light source
        let light_vector = (light.position() - position).normalize();

        // compute the ambient contribution
        let ambient = effective_color * *material.ambient;

        let mut diffuse = Color::default();
        let mut specular = Color::default();

        // Light_dot_normal represents the cosine of the angle between the light vector and the normal vector
        // A negative numer means the light is on the other side of the surface
        let light_dot_normal = light_vector.dot_product(normal_vector);
        if light_dot_normal < 0f32 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * **material.diffuse() * light_dot_normal;

            // Reflect_dot_eye represents the cosine of the angle between the reflection vector and the eye vector
            // A negative number means the light reflects away from the eye
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot_product(eye_vector);

            if reflect_dot_eye <= 0f32 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(material.shininess());
                specular = *light.intensity() * **material.specular() * factor;
            }
        }

        // Add the three contributions together to get the final shading
        let final_color = ambient + diffuse + specular;
        final_color
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

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new_color(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl_display_for_tuple!(Color);
impl_partial_eq_for_tuple!(Color);
impl_sub_for_tuple!(Color);
impl_neg_for_tuple!(Color);
impl_mul_f32_for_tuple!(Color);
impl_mul_for_tuple!(Color);
impl_div_f32_for_tuple!(Color);
