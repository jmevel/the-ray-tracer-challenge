use crate::{Color, ReflectionValue};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: ReflectionValue,
    pub diffuse: ReflectionValue,
    pub specular: ReflectionValue,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        color: Option<Color>,
        ambient: Option<ReflectionValue>,
        diffuse: Option<ReflectionValue>,
        specular: Option<ReflectionValue>,
        shininess: Option<f32>,
    ) -> Self {
        // In the scenarion 'The default world' in 'world.feature', the test is unclear if we are supposed to use the values from a 'default' material or not
        // For now, I'll assume that's the case and that 'ambient' and 'shininess' have their 'default' values even though they aren't mentioned in the test
        let default = Self::default();
        Self {
            color: color.unwrap_or(default.color),
            ambient: ambient.unwrap_or(default.ambient),
            diffuse: diffuse.unwrap_or(default.diffuse),
            specular: specular.unwrap_or(default.specular),
            shininess: shininess.unwrap_or(default.shininess),
        }
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
