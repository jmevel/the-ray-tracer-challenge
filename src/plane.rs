use uuid::Uuid;

use crate::{
    Intersection, Intersections, Material, Matrix, Ray, Shape, Tuple, Vector, float::EPSILON,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    id: Uuid,
    pub transform: Matrix<4, 4>,
    pub material: Material,
}

impl Plane {
    pub fn new(transformation: Option<Matrix<4, 4>>, material: Option<Material>) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: match transformation {
                Some(transformation) => transformation,
                None => Matrix::identity_matrix(),
            },
            material: match material {
                Some(material) => material,
                None => Material::default(),
            },
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
        if ray.direction().y().abs() < EPSILON {
            return Ok(None);
        }
        let t = -ray.origin().y / ray.direction().y();
        Ok(Some(Intersections {
            0: vec![Intersection::new(t, Shape::Plane(self.clone()))],
        }))
    }

    pub fn normal_at(&self) -> Vector {
        Vector::new_vector(0.0, 1.0, 0.0)
    }
}
