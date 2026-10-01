use crate::{Intersection, Point, Ray, Shape, Vector};

#[derive(Debug)]
pub struct Computations {
    pub t: f32,
    pub object: Shape,
    pub point: Point,
    pub over_point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub inside: bool,
}

impl Computations {
    pub fn from_intersection_and_ray(intersection: &Intersection, ray: &Ray) -> Computations {
        let point = ray.position(intersection.t());
        let eye_vector = -ray.direction();
        let mut normal_vector = intersection.shape().normal_at(&point.clone());

        let inside = {
            if normal_vector.dot_product(&eye_vector) < 0.0 {
                normal_vector = -normal_vector;
                true
            } else {
                false
            }
        };

        // Somehow the global EPSILON value was still too big and lots of black dots were appearing on the floor and walls
        // Raising the value only here fixed the issue. Should I use some f64 in some places? I'm a bit lost
        const SPECIAL_EPSILON: f32 = 0.002;
        let over_point = &point + &(normal_vector * SPECIAL_EPSILON);

        Self {
            t: intersection.t().to_owned(),
            object: intersection.shape().clone(),
            point,
            over_point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}
