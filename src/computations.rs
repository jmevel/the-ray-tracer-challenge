use crate::{Intersection, Object, Point, Ray, Vector};

#[derive(Debug)]
pub struct Computations {
    pub t: f32,
    pub object: Object,
    pub point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub inside: bool,
}

impl Computations {
    pub fn from_intersection_and_ray(intersection: &Intersection, ray: &Ray) -> Computations {
        let point = ray.position(intersection.t());
        let eye_vector = -ray.direction();
        let mut normal_vector = match intersection.object() {
            Object::Sphere(sphere) => sphere.normal_at(&point.clone()),
        };

        let inside = {
            if normal_vector.dot_product(&eye_vector) < 0f32 {
                normal_vector = -normal_vector;
                true
            } else {
                false
            }
        };

        Self {
            t: intersection.t().to_owned(),
            object: intersection.object().to_owned(),
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}
