use std::{f32, fs};

use the_ray_tracer_challenge::{
    Camera, Color, Material, Matrix, Object, Point, PointLight, ReflectionValue, Sphere, Vector,
    World,
};

#[allow(dead_code)]
pub fn putting_it_together() {
    let floor = Sphere::new(
        Some(Matrix::new_scaling(10.0, 0.01, 10.0)),
        Some(Material::new(
            Some(Color::new_color(1.0, 0.9, 0.9)),
            None,
            None,
            Some(ReflectionValue::new(0.0)),
            None,
        )),
    );

    let left_wall = Sphere::new(
        Some(
            Matrix::new_scaling(10.0, 0.01, 10.0)
                .rotate_x(f32::consts::PI / 2.0)
                .rotate_y(-f32::consts::PI / 4.0)
                .translate(0.0, 0.0, 5.0),
        ),
        None,
    );

    let right_wall = Sphere::new(
        Some(
            Matrix::new_scaling(10.0, 0.01, 10.0)
                .rotate_x(f32::consts::PI / 2.0)
                .rotate_y(f32::consts::PI / 4.0)
                .translate(0.0, 0.0, 5.0),
        ),
        None,
    );

    let middle = Sphere::new(
        Some(Matrix::new_translation(-0.5, 1.0, 0.5)),
        Some(Material::new(
            Some(Color::new_color(0.1, 1.0, 0.5)),
            None,
            Some(ReflectionValue::new(0.7)),
            Some(ReflectionValue::new(0.7)),
            None,
        )),
    );

    let right = Sphere::new(
        Some(Matrix::new_scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5)),
        Some(Material::new(
            Some(Color::new_color(0.5, 1.0, 0.1)),
            None,
            Some(ReflectionValue::new(0.7)),
            Some(ReflectionValue::new(0.3)),
            None,
        )),
    );

    let left = Sphere::new(
        Some(Matrix::new_scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75)),
        Some(Material::new(
            Some(Color::new_color(1.0, 0.8, 0.1)),
            None,
            Some(ReflectionValue::new(0.7)),
            Some(ReflectionValue::new(0.3)),
            None,
        )),
    );

    let mut world = World::new();
    world.lights = Some(vec![PointLight::new(
        Point::new_point(-10.0, 10.0, -10.0),
        Color::white(),
    )]);
    world.elements = vec![
        Object::Sphere(floor),
        Object::Sphere(left_wall),
        Object::Sphere(right_wall),
        Object::Sphere(middle),
        Object::Sphere(right),
        Object::Sphere(left),
    ];

    let camera = Camera::new(
        2000,
        1000,
        f32::consts::PI / 3.0,
        Some(Matrix::<4, 4>::view_transform(
            &Point::new_point(0.0, 1.5, -5.0),
            &Point::new_point(0.1, 1.0, 0.0),
            &Vector::new_vector(0.0, 1.0, 0.0),
        )),
    );

    let canvas = camera.render(&world);

    fs::write(
        "./resources/images/chapter 7/chapter7.ppm",
        canvas.convert_to_ppm(),
    )
    .unwrap();
}
