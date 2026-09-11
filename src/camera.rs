use crate::Matrix;

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix<4, 4>,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = f32::tan(field_of_view / 2f32);
        let aspect = hsize as f32 / vsize as f32;
        let (half_width, half_height) = if aspect >= 1f32 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2f32) / hsize as f32;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_matrix(),
            half_width,
            half_height,
            pixel_size,
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

    pub fn half_width(&self) -> f32 {
        self.half_width
    }

    pub fn half_height(&self) -> f32 {
        self.half_height
    }

    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }
}
