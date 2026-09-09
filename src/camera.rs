use crate::Matrix;

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix<4, 4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_matrix(),
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn transform(&self) -> &Matrix<4, 4> {
        &self.transform
    }
}
