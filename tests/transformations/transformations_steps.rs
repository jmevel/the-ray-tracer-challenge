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

#[then(expr = "{word} * {word} = point\\({int}, {int}, {int})")]
fn transform_multiplied_by_point_equals_point(
    world: &mut TransformationWorld,
    transform_name: String,
    point_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = world
        .transformations
        .get(&transform_name)
        .expect(format!("{transform_name} does not exist").as_str());

    let point = world
        .tuples
        .get(&point_name)
        .expect(format!("{point_name} does not exist").as_str());

    let expected = Tuple::new_point(x, y, z);
    assert_eq!(transform * point, expected);
}
