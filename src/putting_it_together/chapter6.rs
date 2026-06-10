use std::fs;

use the_ray_tracer_challenge::{Canvas, Color, Material, Object, Point, PointLight, Ray, Sphere};

#[allow(dead_code)]
pub fn putting_it_together() {
    // start the ray at z = -5
    let ray_origin = Point::new_point(0f32, 0f32, -5f32);

    // put the wall at z = 10
    let wall_z = 10f32;
    let wall_size = 7f32;

    let canvas_pixels = 1000;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);

    // size of a single pixel (in world space unit)
    let pixel_size = wall_size / canvas_pixels as f32;

    // since the wall is centered at origin (because the sphere is at the origin)
    // it means that 'half' describes the minimum and maximum x and y coordinates of the wall
    let half = wall_size / 2f32;

    let mut material = Material::default();
    material.color = Color::new_color(1f32, 0.2, 1f32);
    let mut shape = Sphere::new(None);
    shape.material = material.clone();

    let light_positon = Point::new_point(-10f32, 10f32, -10f32);
    let light_color = Color::white();
    let light = PointLight::new(light_positon, light_color);

    // for each row of pixels in the canvas
    for y in 0..canvas_pixels {
        // computation of the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f32;

        // for each pizel in the row
        for x in 0..canvas_pixels {
            //computation of the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f32;

            // point on the wall that the ray will target
            let position = Point::new_point(world_x, world_y, wall_z);

            let direction = position - ray_origin;
            let ray = Ray::new(ray_origin.clone(), direction.normalize().clone());

            let intersections = shape.intersect(&ray);

            match intersections {
                Ok(intersections) => {
                    if let Some(intersections) = intersections {
                        if let Some(hit) = intersections.hit() {
                            let point = ray.position(hit.t());
                            let Object::Sphere(sphere) = hit.object();
                            let normal = sphere.normal_at(&point);
                            let eye = -ray.direction();

                            let color =
                                Color::lighting(&sphere.material, &light, &point, &eye, &normal);

                            canvas.write_pixel(x, y, color);
                        }
                    }
                }

                Err(e) => eprintln!("{e}"),
            };
        }
    }

    let ppm = canvas.convert_to_ppm();
    fs::write("./resources/images/chapter 6/chapter6.ppm", ppm).unwrap();
}
