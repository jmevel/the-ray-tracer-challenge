use crate::{
    Color, Computations, Intersection, Intersections, Material, Matrix, Object, Point, PointLight,
    Ray, ReflectionValue, Sphere,
};

#[derive(Debug)]
pub struct World {
    pub lights: Option<Vec<PointLight>>,
    pub elements: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        Self {
            lights: None,
            elements: Vec::new(),
        }
    }

    pub fn elements<'a>(&'a self) -> Vec<&'a Object> {
        self.elements.iter().collect()
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Option<Intersections>, String> {
        let mut intersections = self
            .elements
            .iter()
            // TODO: use a match when the Object enum will contain other value(s) than Sphere
            // .map(|e| match e {
            //     Object::Sphere(sphere) => sphere.intersect(ray),
            //     _ => panic!("Not implemented"),
            // })
            .map(|e| {
                let Object::Sphere(sphere) = e;
                sphere.intersect(ray)
            })
            .collect::<Result<Vec<Option<Intersections>>, String>>()? // collect can transform a Vec<Result<T>> into a Result<Vec<T>>
            .into_iter()
            .flatten()
            .flat_map(|inter| inter.0)
            .collect::<Vec<Intersection>>();

        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        if intersections.is_empty() {
            return Ok(None);
        }

        Ok(Some(Intersections { 0: intersections }))
    }

    pub fn shade_hit(&self, computations: &Computations) -> Color {
        let Object::Sphere(sphere) = computations.object;
        let colors = match self.lights.as_ref() {
            Some(lights) => lights
                .iter()
                .map(|light| {
                    Color::lighting(
                        &sphere.material,
                        light,
                        &computations.point,
                        &computations.eye_vector,
                        &computations.normal_vector,
                    )
                })
                .collect::<Vec<Color>>(),
            None => panic!("World does not have any light"),
        };

        colors.into_iter().reduce(|acc, color| acc + color).unwrap()
    }

    pub fn color_at(&self, ray: &Ray) -> Result<Color, String> {
        if let Some(intersections) = self.intersect(ray)? {
            if let Some(hit) = intersections.hit() {
                let computations = Computations::from_intersection_and_ray(hit, ray);
                println!("computations: {:?}", computations);
                return Ok(self.shade_hit(&computations));
            }
        }
        Ok(Color::black())
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
            lights: Some(vec![light]),
            elements,
        }
    }
}
