use crate::{Intersection, Object, Point, Ray, Vector};

#[derive(Debug)]
pub struct Computations<'a> {
    pub t: &'a f32,
    pub object: &'a Object,
    pub point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
}

impl<'a> Computations<'a> {
    pub fn from_intersection_and_ray(
        intersection: &'a Intersection,
        ray: &'a Ray,
    ) -> Computations<'a> {
        let point = ray.position(intersection.t());
        let normal_vector = match intersection.object() {
            Object::Sphere(sphere) => sphere.normal_at(&point.clone()),
        };
        Self {
            t: intersection.t(),
            object: intersection.object(),
            point,
            eye_vector: -ray.direction(),
            normal_vector,
        }
    }

    pub fn t(&self) -> &'a f32 {
        self.t
    }

    pub fn object(&self) -> &'a Object {
        self.object
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn eye_vector(&self) -> Vector {
        self.eye_vector
    }

    pub fn normal_vector(&self) -> Vector {
        self.normal_vector
    }
}
