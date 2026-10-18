use uuid::Uuid;

use crate::{Intersections, Material, Matrix, Plane, Point, Ray, Sphere, TestShape, Vector};

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    TestShape(TestShape),
}

impl Shape {
    pub fn id(&self) -> &Uuid {
        match self {
            Self::Sphere(sphere) => &sphere.id(),
            Self::Plane(plane) => &plane.id(),
            Self::TestShape(test_shape) => test_shape.id(),
        }
    }

    pub fn transform(&self) -> &Matrix<4, 4> {
        match self {
            Self::Sphere(sphere) => &sphere.transform(),
            Self::Plane(plane) => &plane.transform(),
            Self::TestShape(test_shape) => test_shape.transform(),
        }
    }

    pub fn set_transform(&mut self, transformation: Matrix<4, 4>) {
        match self {
            Shape::Sphere(sphere) => sphere.transform = transformation,
            Shape::Plane(plane) => plane.transform = transformation,
            Shape::TestShape(test_shape) => test_shape.transform = transformation,
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => sphere.material(),
            Self::Plane(plane) => plane.material(),
            Self::TestShape(test_shape) => test_shape.material(),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(sphere) => sphere.material = material,
            Shape::Plane(plane) => plane.material = material,
            Shape::TestShape(test_shape) => test_shape.material = material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Option<Intersections>, String> {
        // If the shape has been transformed, the inverse of the transformation must be applied to the ray as well before calculating the intersections
        let ray = ray.transform(self.transform().invert().as_ref()?);

        match self {
            Self::Sphere(sphere) => sphere.intersect(&ray),
            Self::Plane(plane) => plane.intersect(&ray),
            Self::TestShape(test_shape) => test_shape.intersect(),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Result<Vector, String> {
        let transformation = self.transform().invert()?;
        let local_point = &transformation * world_point;
        let local_normal = match self {
            Self::Sphere(sphere) => sphere.normal_at(&local_point),
            Self::Plane(plane) => plane.normal_at(),
            Self::TestShape(test_shape) => test_shape.normal_at(&local_point),
        };
        let world_normal = transformation.transpose() * local_normal;
        Ok(world_normal.with_w(0.0).normalize())
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sphere(this_sphere), Self::Sphere(that_sphere)) => this_sphere == that_sphere,
            (Self::Plane(this_plane), Self::Plane(that_plane)) => this_plane == that_plane,
            (Self::TestShape(this_test_shape), Self::TestShape(that_test_shape)) => {
                this_test_shape == that_test_shape
            }
            _ => panic!("These two shapes aren't of the same type, can't compare them"),
        }
    }
}
