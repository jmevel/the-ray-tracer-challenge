use std::panic;

use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Vector, float::float_equals};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← vector\\({float}, √{float}\\/{float}, -√{float}\\/{float})")]
fn vector_is_vector(
    world: &mut RayTracerWorld,
    vector_name: String,
    x: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = -f32::sqrt(z_numerator) / z_denominator;
    let vector = Vector::new_vector(x, y, z);
    world.add_vector(vector_name, vector);
}

#[given(expr = "{word} ← vector\\({float}, -√{float}\\/{float}, -√{float}\\/{float})")]
fn vector_is_vector2(
    world: &mut RayTracerWorld,
    vector_name: String,
    x: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let y = -f32::sqrt(y_numerator) / y_denominator;
    let z = -f32::sqrt(z_numerator) / z_denominator;
    let vector = Vector::new_vector(x, y, z);
    world.add_vector(vector_name, vector);
}

#[when(expr = "{word} ← reflect\\({word}, {word})")]
fn vector_is_reflect_of_vector_and_vector(
    world: &mut RayTracerWorld,
    result_name: String,
    in_vector: String,
    normal_vector: String,
) {
    let in_vector = world.get_vector(&in_vector);
    let normal_vector = world.get_vector(&normal_vector);
    let result = in_vector.reflect(normal_vector);
    world.add_vector(result_name, result);
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

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut RayTracerWorld, tuple: String) {
    _ = world.get_vector(&tuple);
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut RayTracerWorld, tuple: String) {
    let result = panic::catch_unwind(|| world.get_vector(&tuple));
    assert!(result.is_err());
}

#[then(expr = "magnitude\\({word}) = {float}")]
fn magnitude_equals_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let tuple = world.get_vector(&tuple);

    assert!(float_equals(&tuple.magnitude(), &expected));
}

#[then(expr = "magnitude\\({word}) = √{float}")]
fn magnitude_equals_squareroot_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let expected = expected.sqrt();
    let tuple = world.get_vector(&tuple);

    assert_eq!(tuple.magnitude(), expected);
}

#[then(expr = "normalize\\({word}) = vector\\({float}, {float}, {float})")]
fn normalize_equals_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    let expected = Vector::new_vector(x, y, z);
    let tuple = world.get_vector(&tuple);

    assert_eq!(tuple.normalize(), expected);
}

#[then(expr = "normalize\\({word}) = approximately vector\\({float}, {float}, {float})")]
fn normalize_equals_approximately_vector(
    world: &mut RayTracerWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
) {
    normalize_equals_vector(world, tuple, x, y, z);
}

#[then(expr = "dot\\({word}, {word}) = {float}")]
fn dot_two_vectors_equals(
    world: &mut RayTracerWorld,
    vector1: String,
    vector2: String,
    expected: f32,
) {
    let vector1 = world.get_vector(&vector1);
    let vector2 = world.get_vector(&vector2);

    assert_eq!(vector1.dot_product(vector2), expected);
}

#[then(expr = "cross\\({word}, {word}) = vector\\({float}, {float}, {float})")]
fn cross_two_vectors_equals_vector(
    world: &mut RayTracerWorld,
    vector1: String,
    vector2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let vector1 = world.get_vector(&vector1);
    let vector2 = world.get_vector(&vector2);
    let expected = Vector::new_vector(x, y, z);

    assert_eq!(vector1.cross_product(vector2), expected);
}
