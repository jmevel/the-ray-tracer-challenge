use uuid::Uuid;

use crate::{Intersections, Material, Matrix, Point, Ray, Sphere, TestShape, Vector};

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    TestShape(TestShape),
}

impl Shape {
    pub fn id(&self) -> &Uuid {
        match self {
            Self::Sphere(sphere) => &sphere.id(),
            Self::TestShape(test_shape) => test_shape.id(),
        }
    }

    pub fn transform(&self) -> &Matrix<4, 4> {
        match self {
            Self::Sphere(sphere) => &sphere.transform(),
            Self::TestShape(test_shape) => test_shape.transform(),
        }
    }

    pub fn set_transform(&mut self, transformation: Matrix<4, 4>) {
        match self {
            Shape::Sphere(sphere) => sphere.transform = transformation,
            Shape::TestShape(test_shape) => test_shape.transform = transformation,
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => sphere.material(),
            Self::TestShape(test_shape) => test_shape.material(),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(sphere) => sphere.material = material,
            Shape::TestShape(test_shape) => test_shape.material = material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Option<Intersections>, String> {
        match self {
            Self::Sphere(sphere) => sphere.intersect(ray),
            Self::TestShape(test_shape) => test_shape.intersect(),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        match self {
            Self::Sphere(sphere) => self.normal_at_common(world_point, sphere.transform()),
            Self::TestShape(test_shape) => {
                self.normal_at_common(world_point, test_shape.transform())
            }
        }
    }

    fn normal_at_common(&self, world_point: &Point, transform: &Matrix<4, 4>) -> Vector {
        let object_point = &transform.invert().unwrap() * world_point;
        let object_normal = object_point - Point::new_point(0.0, 0.0, 0.0);
        let world_normal = &transform.invert().unwrap().transpose() * &object_normal;
        world_normal.with_w(0.0).normalize()
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sphere(this_sphere), Self::Sphere(that_sphere)) => this_sphere == that_sphere,
            (Self::TestShape(this_test_shape), Self::TestShape(that_test_shape)) => {
                this_test_shape == that_test_shape
            }
            _ => panic!("These two shapes aren't of the same type, can't compare them"),
        }
    }
}
