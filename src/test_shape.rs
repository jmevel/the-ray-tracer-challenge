use uuid::Uuid;

use crate::{Intersections, Material, Matrix, Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {
    id: Uuid,
    pub transform: Matrix<4, 4>,
    pub material: Material,
}

impl TestShape {
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

    pub fn intersect(&self) -> Result<Option<Intersections>, String> {
        Ok(None)
    }

    pub fn normal_at(&self, local_point: &Point) -> Vector {
        Vector::new_vector(local_point.x, local_point.y, local_point.z)
    }
}
