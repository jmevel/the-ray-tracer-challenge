use crate::RayTracerWorld;
use cucumber::{given, then};
use std::f32;
use the_ray_tracer_challenge::{Matrix, Point, Vector};

#[given(expr = "{word} ← translation\\({int}, {int}, {int})")]
fn transform_is_translation(
    world: &mut RayTracerWorld,
    translation: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = Matrix::new_translation(x, y, z);
    world.add_matrix4x4(translation.clone(), transform);
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float})")]
fn transform_is_scaling(world: &mut RayTracerWorld, scaling: String, x: f32, y: f32, z: f32) {
    let transform = Matrix::new_scaling(x, y, z);
    world.add_matrix4x4(scaling.clone(), transform);
}

#[given(expr = "{word} ← rotation_x\\(π \\/ {float})")]
fn rotation_x_is(world: &mut RayTracerWorld, rotation: String, denominator: f32) {
    let fraction = f32::consts::PI / denominator;
    let transform = Matrix::new_rotation_x(fraction);
    world.add_matrix4x4(rotation.clone(), transform);
}

#[given(expr = "{word} ← rotation_y\\(π \\/ {float})")]
fn rotation_y_is(world: &mut RayTracerWorld, rotation: String, denominator: f32) {
    let fraction = f32::consts::PI / denominator;
    let transform = Matrix::new_rotation_y(fraction);
    world.add_matrix4x4(rotation.clone(), transform);
}

#[given(expr = "{word} ← rotation_z\\(π \\/ {float})")]
fn rotation_z_is(world: &mut RayTracerWorld, rotation: String, denominator: f32) {
    let fraction = f32::consts::PI / denominator;
    let transform = Matrix::new_rotation_z(fraction);
    world.add_matrix4x4(rotation.clone(), transform);
}

#[given(expr = "{word} ← shearing\\({float}, {float}, {float}, {float}, {float}, {float})")]
fn shearing_is(
    world: &mut RayTracerWorld,
    shearing: String,
    xy: f32,
    xz: f32,
    yx: f32,
    yz: f32,
    zx: f32,
    zy: f32,
) {
    let transform = Matrix::new_shearing(xy, xz, yx, yz, zx, zy);
    world.add_matrix4x4(shearing.clone(), transform);
}

#[then(expr = "{word} * {word} = point\\({int}, {int}, {int})")]
fn transform_multiplied_by_point_equals_point(
    world: &mut RayTracerWorld,
    transformation: String,
    point_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transformation = world.get_matrix4x4(&transformation);
    let point = world.get_point(&point_name);
    let expected = Point::new_point(x, y, z);

    assert_eq!(transformation * point, expected);
}

#[then(expr = "{word} * {word} = vector\\({int}, {int}, {int})")]
fn transformation_multiplied_by_vector_equals_vector(
    world: &mut RayTracerWorld,
    transformation: String,
    vector: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transformation = world.get_matrix4x4(&transformation);
    let vector = world.get_vector(&vector);
    let expected = Vector::new_vector(x, y, z);
    let actual = transformation * vector;

    assert_eq!(actual, expected);
}

#[then(expr = "{word} * {word} = point\\({int}, √{int}\\/{int}, √{int}\\/{int})")]
fn rotation_multiplied_by_point_equals_point(
    world: &mut RayTracerWorld,
    rotation: String,
    point: String,
    x: f32,
    y_numerator: i32,
    y_denominator: i32,
    z_numerator: i32,
    z_denominator: i32,
) {
    let y = f32::sqrt(y_numerator as f32) / y_denominator as f32;
    let z = f32::sqrt(z_numerator as f32) / z_denominator as f32;
    let expected = Point::new_point(x, y, z);

    point_multiplied_by_transformation_equals_expected(world, point, rotation, expected);
}

#[then(expr = "{word} * {word} = point\\(√{int}\\/{int}, {int}, √{int}\\/{int})")]
fn rotation_multiplied_by_point_equals_point2(
    world: &mut RayTracerWorld,
    rotation: String,
    point: String,
    x_numerator: f32,
    x_denominator: i32,
    y: f32,
    z_numerator: i32,
    z_denominator: i32,
) {
    let x = f32::sqrt(x_numerator as f32) / x_denominator as f32;
    let z = f32::sqrt(z_numerator as f32) / z_denominator as f32;
    let expected = Point::new_point(x, y, z);

    point_multiplied_by_transformation_equals_expected(world, point, rotation, expected);
}

#[then(expr = "{word} * {word} = point\\(-√{int}\\/{int}, √{int}\\/{int}, {int})")]
fn rotation_multiplied_by_point_equals_point3(
    world: &mut RayTracerWorld,
    rotation: String,
    point: String,
    x_numerator: f32,
    x_denominator: i32,
    y_numerator: f32,
    y_denominator: i32,
    z: f32,
) {
    let x = f32::sqrt(x_numerator as f32) / x_denominator as f32;
    let y = f32::sqrt(y_numerator as f32) / y_denominator as f32;
    let expected = Point::new_point(-x, y, z);

    point_multiplied_by_transformation_equals_expected(world, point, rotation, expected);
}

#[then(expr = "{word} * {word} = point\\({int}, √{int}\\/{int}, -√{int}\\/{int})")]
fn inverse_of_rotation_multiplied_by_point_equals_point(
    world: &mut RayTracerWorld,
    rotation: String,
    point: String,
    x: f32,
    y_numerator: i32,
    y_denominator: i32,
    z_numerator: i32,
    z_denominator: i32,
) {
    let y = f32::sqrt(y_numerator as f32) / y_denominator as f32;
    let z = f32::sqrt(z_numerator as f32) / z_denominator as f32;
    let expected = Point::new_point(x, y, -z);

    point_multiplied_by_transformation_equals_expected(world, point, rotation, expected);
}

fn point_multiplied_by_transformation_equals_expected(
    world: &mut RayTracerWorld,
    point: String,
    transformation: String,
    expected: Point,
) {
    let transformation = world.get_matrix4x4(&transformation);
    let point = world.get_point(&point);

    assert_eq!(point * transformation, expected);
}
