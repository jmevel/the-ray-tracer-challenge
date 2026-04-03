#[path = "../common/mod.rs"]
mod common;

use common::ray_tracer_world::RayTracerWorld;

use cucumber::{World, given, then};
use std::f32;
use the_ray_tracer_challenge::{Matrix, Tuple};

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
pub fn tuple_is_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(tuple, Tuple::new_vector(x, y, z));
}

#[given(expr = "{word} ← translation\\({int}, {int}, {int})")]
fn transform_is_translation(
    world: &mut RayTracerWorld,
    translation: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = Matrix::translation(x, y, z);
    world.matrices4x4.insert(translation.clone(), transform);
}

#[given(expr = "{word} ← point\\({int}, {int}, {int})")]
fn p_is_point(world: &mut RayTracerWorld, point_name: String, x: f32, y: f32, z: f32) {
    let point = Tuple::new_point(x, y, z);
    world.tuples.insert(point_name, point);
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float})")]
fn transform_is_scaling(world: &mut RayTracerWorld, scaling: String, x: f32, y: f32, z: f32) {
    let transform = Matrix::scaling(x, y, z);
    world.add_matrix4x4(scaling.clone(), transform);
}

#[given(expr = "{word} ← rotation_x\\(π \\/ {float})")]
fn rotation_is(world: &mut RayTracerWorld, rotation: String, denominator: f32) {
    let fraction = f32::consts::PI / denominator;
    let transform = Matrix::rotation_x(fraction);
    world.add_matrix4x4(rotation.clone(), transform);
}

#[given(expr = "{word} ← inverse\\({word})")]
fn matrix_is_inverse_of_matrix(
    world: &mut RayTracerWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
) {
    match world
        .matrices
        .get(&initial_matrix_name)
        .expect(format!("{initial_matrix_name} does not exist").as_str())
    {
        (2, 2) => panic!("can't invert a 2x2 matrix"),
        (3, 3) => {
            let initial_matrix = world.get_matrix3x3(&initial_matrix_name);
            world.add_matrix3x3(new_matrix_name, initial_matrix.invert().unwrap());
        }
        (4, 4) => {
            let initial_matrix = world.get_matrix4x4(&initial_matrix_name);
            world.add_matrix4x4(new_matrix_name, initial_matrix.invert().unwrap());
        }
        _ => panic!("no matrix with given size"),
    }
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
    let point = world.get_tuple(&point_name);
    let expected = Tuple::new_point(x, y, z);

    assert_eq!(transformation * point, expected);
}

#[then(expr = "{word} * {word} = {word}")]
fn transformation_multiplied_by_vector_equals_same_vector(
    world: &mut RayTracerWorld,
    transformation: String,
    vector: String,
    _same_vector: String,
) {
    let transformation = world.get_matrix4x4(&transformation);
    let vector = world.get_tuple(&vector);
    let actual = transformation * vector;

    assert_eq!(&actual, vector);
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
    let vector = world.get_tuple(&vector);
    let expected = Tuple::new_vector(x, y, z);
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
    let expected = Tuple::new_point(x, y, z);

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
    let expected = Tuple::new_point(x, y, -z);

    point_multiplied_by_transformation_equals_expected(world, point, rotation, expected);
}

fn point_multiplied_by_transformation_equals_expected(
    world: &mut RayTracerWorld,
    point: String,
    transformation: String,
    expected: Tuple,
) {
    let transformation = world.get_matrix4x4(&transformation);
    let point = world.get_tuple(&point);

    assert_eq!(transformation * point, expected);
}

#[tokio::main]
async fn main() {
    RayTracerWorld::run("tests/features/transformations.feature").await;
}
