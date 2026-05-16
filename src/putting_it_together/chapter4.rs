use std::{f32, fs};

use the_ray_tracer_challenge::{Canvas, Matrix, Tuple};

#[allow(dead_code)]
pub fn putting_it_together() {
    let mut canvas = Canvas::new(500, 500, None);
    let white = Tuple::new_color(255f32, 255f32, 255f32);
    let red = Tuple::new_color(255f32, 0f32, 0f32);

    let origin = Tuple::new_point(0f32, 0f32, 0f32);
    let middle_translation: Matrix<4, 4> = Matrix::new_translation(
        *canvas.width() as f32 / 2f32,
        *canvas.height() as f32 / 2f32,
        0f32,
    );
    let origin: Tuple = origin.transform(&middle_translation);
    canvas.write_pixel(
        *origin.x() as usize,
        canvas.height() - *origin.y() as usize,
        red,
    );

    let radius: f32 = (3f32 / 8f32) * *canvas.width() as f32;
    let twelve = Tuple::new_point(0f32, 1f32, 0f32);

    for hour in 1..=12 {
        // Equivalent to:
        // let rotation: Matrix<4, 4> = Matrix::new_rotation_z(hour as f32 * (f32::consts::PI / 6f32));
        // let point = &twelve * &rotation;
        // let point = Tuple::new_point(point.x() * radius, point.y() * radius, 0f32);
        // let point = Tuple::new_point(point.x() + origin.x(), point.y() + origin.y(), 0f32);

        let transform: Matrix<4, 4> = Matrix::new_translation(0f32, radius, 0f32)
            .rotate_z(hour as f32 * (f32::consts::PI / 6f32))
            .translate(*origin.x(), *origin.y(), 0f32);
        let point = twelve.transform(&transform);
        canvas.write_pixel(
            *point.x() as usize,
            *canvas.height() - *point.y() as usize,
            white.clone(),
        );
    }
    let ppm = canvas.convert_to_ppm();
    fs::write("./resources/images/chapter 4/chapter4.ppm", ppm).unwrap();
}
