use crate::{Matrix, Point, Ray};

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    pub transform: Matrix<4, 4>,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = f32::tan(field_of_view / 2.0);
        let aspect = hsize as f32 / vsize as f32;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f32;

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

    pub fn ray_for_pixel(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        // The offset from the edge of the camera to the pixel's center
        let offset_x = (pixel_x as f32 + 0.5) * self.pixel_size();
        let offset_y = (pixel_y as f32 + 0.5) * self.pixel_size();

        // The untransformed coordinates of the pixel in world space
        // REMINDER: unlike the book using the -Z convention, in this project the camera looks toward +Z, so +X is to the right
        let world_x = self.half_width() - offset_x;
        let world_y = self.half_height() - offset_y;

        // Using the camera matrix, transform the canvas point and the origin and then compute the ray's direction vector
        // REMINDER: unlike in the book in which the canvas is at Z=-1, here the canvas is at Z=+1
        let pixel = self.transform().invert().unwrap() * Point::new_point(world_x, world_y, 1.0);
        let origin = self.transform().invert().unwrap() * Point::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}
