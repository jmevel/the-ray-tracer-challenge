use std::f32;

use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Matrix, Point, Sphere, Vector};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← sphere\\()")]
fn sphere_is(world: &mut RayTracerWorld, sphere_name: String) {
    let sphere = Sphere::new(None);
    world.add_sphere(sphere_name, sphere);
}

#[when(expr = "{word} ← intersect\\({word}, {word})")]
fn intersection_is_intersect_of_sphere_and_ray(
    world: &mut RayTracerWorld,
    intersect_name: String,
    sphere_name: String,
    ray_name: String,
) -> Result<(), String> {
    let sphere = world.get_sphere(&sphere_name);
    let ray = world.get_ray(&ray_name);
    let intersections = sphere.intersect(ray);
    world.add_intersections_collection(intersect_name, intersections?);
    Ok(())
}

#[when(expr = "set_transform\\({word}, {word})")]
#[given(expr = "set_transform\\({word}, {word})")]
fn set_transform_s_t(world: &mut RayTracerWorld, sphere: String, transformation: String) {
    let transformation = world.get_matrix4x4(&transformation);
    world.get_mut_sphere(&sphere).transform = transformation.clone();
}

#[when(expr = "{word} ← normal_at\\({word}, point\\({float}, {float}, {float}))")]
fn n_normal_at_point(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let sphere = world.get_sphere(&sphere);
    let point = Point::new_point(x, y, z);
    let normal = sphere.normal_at(&point);
    world.add_vector(normal_name, normal);
}

#[when(
    expr = "{word} ← normal_at\\({word}, point\\(√{float}\\/{float}, √{float}\\/{float}, √{float}\\/{float}))"
)]
fn n_normal_at_point2(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x_numerator: f32,
    x_denominator: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let x = f32::sqrt(x_numerator) / x_denominator;
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = f32::sqrt(z_numerator) / z_denominator;
    n_normal_at_point(world, normal_name, sphere, x, y, z);
}

#[when(
    expr = "{word} ← normal_at\\({word}, point\\({float}, √{float}\\/{float}, -√{float}\\/{float}))"
)]
fn n_normal_at_point3(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = -f32::sqrt(z_numerator) / z_denominator;
    n_normal_at_point(world, normal_name, sphere, x, y, z);
}

#[then(expr = "{word}.transform = {word}")]
fn sphere_transform_equals_identity_matrix(
    world: &mut RayTracerWorld,
    sphere: String,
    transformation: String,
) {
    let sphere = world.get_sphere(&sphere);
    let expected = match transformation.as_str() {
        "identity_matrix" => &Matrix::identity_matrix(),
        _ => world.get_matrix4x4(&transformation),
    };
    assert_eq!(&sphere.transform, expected);
}

#[then(
    regex = r"^([a-zA-Z0-9]*) = vector\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn vector_equals_vector(world: &mut RayTracerWorld, vector: String, x: f32, y: f32, z: f32) {
    let actual = world.get_vector(&vector);
    let expected = Vector::new_vector(x, y, z);
    assert_eq!(actual, &expected);
}

#[then(expr = "{word} = vector\\(√{float}\\/{float}, √{float}\\/{float}, √{float}\\/{float})")]
fn vector_equals_vector2(
    world: &mut RayTracerWorld,
    vector: String,
    x_numerator: f32,
    x_denominator: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let x = f32::sqrt(x_numerator) / x_denominator;
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = f32::sqrt(z_numerator) / z_denominator;
    vector_equals_vector(world, vector, x, y, z);
}

#[then(expr = "{word} = normalize\\({word})")]
fn vector_equals_vector_normalized(world: &mut RayTracerWorld, vector: String, _vector: String) {
    let expected = world.get_vector(&vector);
    let actual = expected.normalize();
    assert_eq!(&actual, expected);
}

#[given(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}))")]
fn set_transform_s_translation(world: &mut RayTracerWorld, sphere: String, x: f32, y: f32, z: f32) {
    let sphere = world.get_mut_sphere(&sphere);
    let transformation = Matrix::new_translation(x, y, z);
    sphere.transform = transformation;
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float}) * rotation_z\\(π\\/{float})")]
fn transformation_is_scaling_and_rotation_z(
    world: &mut RayTracerWorld,
    transformation_name: String,
    x: f32,
    y: f32,
    z: f32,
    denominator: f32,
) {
    let transformation: Matrix<4, 4> =
        Matrix::new_rotation_z(f32::consts::PI / denominator).scale(x, y, z);
    world.add_matrix4x4(transformation_name, transformation);
}
