use crate::{Color, Material, Matrix, Object, Point, PointLight, ReflectionValue, Sphere};

#[derive(Debug)]
pub struct World {
    pub light: Option<PointLight>,
    pub elements: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        Self {
            light: None,
            elements: Vec::new(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light_position = Point::new_point(-10.0, 10.0, -10.0);
        let light = PointLight::new(light_position, Color::white());

        let mut s1 = Sphere::new(None);
        let s1_material = Material::new(
            Some(Color::new_color(0.8, 1.0, 0.6)),
            None,
            Some(ReflectionValue::new(0.7)),
            Some(ReflectionValue::new(0.2)),
            None,
        );
        s1.material = s1_material;

        let s2 = Sphere::new(Some(Matrix::new_scaling(0.5, 0.5, 0.5)));
        let elements: Vec<Object> = vec![Object::Sphere(s1), Object::Sphere(s2)];

        Self {
            light: Some(light),
            elements,
        }
    }
}
