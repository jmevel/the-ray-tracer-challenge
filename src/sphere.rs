use uuid::Uuid;

use crate::{Intersection, Intersections, Material, Matrix, Object, Point, Ray, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix<4, 4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(transformation: Option<Matrix<4, 4>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: match transformation {
                Some(transformation) => transformation,
                None => Matrix::identity_matrix(),
            },
            material: Material::default(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn transform(&self) -> &Matrix<4, 4> {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Option<Intersections>, String> {
        // If the sphere has been transformed, the inverse of the transformation must be applied to the ray as well before calculating the intersections
        let ray = ray.transform(&self.transform.invert()?);

        let sphere_to_ray = ray.origin() - &Point::new_point(0f32, 0f32, 0f32);
        let a = ray.direction().dot_product(ray.direction());
        let b = 2f32 * ray.direction().dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1f32;
        let discriminant = b.powi(2) - (4f32 * a * c);

        if discriminant < 0f32 {
            return Ok(None);
        }

        let t1 = (-b - discriminant.sqrt()) / (2f32 * a);
        let t2 = (-b + discriminant.sqrt()) / (2f32 * a);

        Ok(Some(Intersections {
            0: vec![
                Intersection::new(t1, Object::Sphere(self.clone())),
                Intersection::new(t2, Object::Sphere(self.clone())),
            ],
        }))
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = &self.transform.invert().unwrap() * world_point;
        let object_normal = object_point - Point::new_point(0f32, 0f32, 0f32);
        let world_normal = &self.transform.invert().unwrap().transpose() * &object_normal;

        world_normal.with_w(0f32).normalize()
    }
}
