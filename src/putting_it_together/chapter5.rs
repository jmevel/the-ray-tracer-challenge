use std::{f32, fs, thread};

use the_ray_tracer_challenge::{Canvas, Matrix, Ray, Sphere, Tuple};

#[allow(dead_code)]
pub fn putting_it_together() {
    let sphere_thread = thread::spawn(move || {
        show_sphere_shadow_on_a_wall(None, "chapter5");
        println!("sphere finished");
    });

    // shrinked along the y axis
    let shrink_y_axis_thread = thread::spawn(move || {
        show_sphere_shadow_on_a_wall(
            Some(Matrix::new_scaling(1f32, 0.5, 1f32)),
            "chapter5_shrinked_y_axis",
        );
        println!("shrinked along the y axis finished");
    });

    // shrinked along the x axis
    let shrink_x_axis_thread = thread::spawn(move || {
        show_sphere_shadow_on_a_wall(
            Some(Matrix::new_scaling(0.5, 1f32, 1f32)),
            "chapter5_shrinked_x_axis",
        );
        println!("shrinked along the x axis finished");
    });

    // shrinked and rotated
    let shrink_and_rotate_tread = thread::spawn(move || {
        let transformation = Matrix::new_scaling(0.5, 1f32, 1f32).rotate_z(f32::consts::PI / 4f32);
        show_sphere_shadow_on_a_wall(Some(transformation), "chapter5_shrinked_and_rotated");
        println!("shrinked and rotated finished");
    });

    // shrinked and skewed
    let shrink_and_skew_thread = thread::spawn(move || {
        let transformation =
            Matrix::new_scaling(0.5, 1f32, 1f32).shear(1f32, 0f32, 0f32, 0f32, 0f32, 0f32);
        show_sphere_shadow_on_a_wall(Some(transformation), "chapter5_shrinked_and_skewed");
        println!("shrinked and skewed finished");
    });

    sphere_thread.join().unwrap();
    shrink_y_axis_thread.join().unwrap();
    shrink_x_axis_thread.join().unwrap();
    shrink_and_rotate_tread.join().unwrap();
    shrink_and_skew_thread.join().unwrap();
}

fn show_sphere_shadow_on_a_wall(transformation: Option<Matrix<4, 4>>, file_name: &str) {
    // start the ray at z = -5
    let ray_origin = Tuple::new_point(0f32, 0f32, -5f32);

    // put the wall at z = 10
    let wall_z = 10f32;
    let wall_size = 7f32;

    let canvas_pixels = 500;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);

    // size of a single pixel (in world space unit)
    let pixel_size = wall_size / canvas_pixels as f32;

    // since the wall is centered at origin (because the sphere is at the origin)
    // it means that 'half' describes the minimum and maximum x and y coordinates of the wall
    let half = wall_size / 2f32;

    let color = Tuple::new_color(1f32, 0f32, 0f32);
    let shape = Sphere::new(transformation);

    // for each row of pixels in the canvas
    for y in 0..canvas_pixels {
        // computation of the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f32;

        // for each pizel in the row
        for x in 0..canvas_pixels {
            //computation of the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f32;

            // point on the wall that the ray will target
            let position = Tuple::new_point(world_x, world_y, wall_z);

            let direction = position - ray_origin;
            let ray = Ray::new(ray_origin.clone(), direction.normalize().clone());

            let intersections = shape.intersect(&ray);

            match intersections {
                Ok(intersections) => {
                    if let Some(intersections) = intersections {
                        if let Some(_) = intersections.hit() {
                            canvas.write_pixel(x, y, color);
                        }
                    }
                }
                Err(e) => eprintln!("{e}"),
            };
        }
    }

    let ppm = canvas.convert_to_ppm();
    fs::write(
        format!("./resources/images/chapter 5/{}.ppm", file_name),
        ppm,
    )
    .unwrap();
}
