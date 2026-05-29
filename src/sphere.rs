use uuid::Uuid;

use crate::{Intersection, Object, Ray, Tuple, intersections::Intersections};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sphere {
    id: Uuid,
}

impl Sphere {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersections> {
        let sphere_to_ray = ray.origin() - &Tuple::new_point(0f32, 0f32, 0f32);
        let a = ray.direction().dot_product(ray.direction());
        let b = 2f32 * ray.direction().dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1f32;
        let discriminant = b.powi(2) - (4f32 * a * c);

        if discriminant < 0f32 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2f32 * a);
        let t2 = (-b + discriminant.sqrt()) / (2f32 * a);

        Some(Intersections::new(vec![
            Intersection::new(t1, Object::Sphere(self.clone())),
            Intersection::new(t2, Object::Sphere(self.clone())),
        ]))
    }
}
