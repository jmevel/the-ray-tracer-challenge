use crate::{Color, ReflectionValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: ReflectionValue,
    diffuse: ReflectionValue,
    specular: ReflectionValue,
    shininess: f32,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: ReflectionValue,
        diffuse: ReflectionValue,
        specular: ReflectionValue,
        shininess: f32,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn diffuse(&self) -> &ReflectionValue {
        &self.diffuse
    }

    pub fn specular(&self) -> &ReflectionValue {
        &self.specular
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new_color(1f32, 1f32, 1f32),
            ambient: ReflectionValue::new(0.1),
            diffuse: ReflectionValue::new(0.9),
            specular: ReflectionValue::new(0.9),
            shininess: 200f32,
        }
    }
}
