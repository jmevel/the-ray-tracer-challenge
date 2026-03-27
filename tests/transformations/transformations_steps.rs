use std::collections::HashMap;

use cucumber::{World, given, then};
use the_ray_tracer_challenge::{Matrix, Tuple};

#[derive(Debug, Default, World)]
pub struct TransformationWorld {
    transformations: HashMap<String, Matrix<4, 4>>,
    tuples: HashMap<String, Tuple>,
}

#[given(expr = "{word} ← translation\\({int}, {int}, {int})")]
fn transform_is_translation(
    world: &mut TransformationWorld,
    translation: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = Matrix::translation(x, y, z);
    world.transformations.insert(translation.clone(), transform);
}

#[given(expr = "{word} ← point\\({int}, {int}, {int})")]
fn p_is_point(world: &mut TransformationWorld, point_name: String, x: f32, y: f32, z: f32) {
    let point = Tuple::new_point(x, y, z);
    world.tuples.insert(point_name, point);
}

#[given(expr = "{word} ← inverse\\({word})")]
fn matrix_is_inverse_of_matrix(
    world: &mut TransformationWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
) {
    let initial_matrix = world
        .transformations
        .get(&initial_matrix_name)
        .expect(format!("{initial_matrix_name} does not exist").as_str());

    world
        .transformations
        .insert(new_matrix_name.clone(), initial_matrix.invert().unwrap());
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn tuple_is_vector(world: &mut TransformationWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(tuple, Tuple::new_vector(x, y, z));
}

#[then(expr = "{word} * {word} = point\\({int}, {int}, {int})")]
fn transform_multiplied_by_point_equals_point(
    world: &mut TransformationWorld,
    transformation: String,
    point_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transformation = world
        .transformations
        .get(&transformation)
        .expect(format!("{transformation} does not exist").as_str());

    let point = world
        .tuples
        .get(&point_name)
        .expect(format!("{point_name} does not exist").as_str());

    let expected = Tuple::new_point(x, y, z);
    assert_eq!(transformation * point, expected);
}

#[then(expr = "{word} * {word} = {word}")]
fn transformation_multiplied_by_vector_equals_same_vector(
    world: &mut TransformationWorld,
    transformation: String,
    vector: String,
    _same_vector: String,
) {
    let transformation = world
        .transformations
        .get(&transformation)
        .expect(format!("{transformation} does not exist").as_str());

    let vector = world
        .tuples
        .get(&vector)
        .expect(format!("{vector} does not exist").as_str());

    let actual = transformation * vector;

    assert_eq!(&actual, vector);
}
