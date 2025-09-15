use cucumber::{given, then, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::ExtendedTuple;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuples: HashMap<String, ExtendedTuple>,
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    world
        .tuples
        .insert(tuple_name, ExtendedTuple::new(x, y, z, w));
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn point(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32) {
    world
        .tuples
        .insert(tuple_name, ExtendedTuple::new_point(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn vector(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32) {
    world
        .tuples
        .insert(tuple_name, ExtendedTuple::new_vector(x, y, z));
}

#[then(expr = "{word}.x = {float}")]
fn x_equal(world: &mut TupleWorld, tuple_name: String, x: f32) {
    assert_eq!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .0,
        x
    );
}

#[then(expr = "{word}.y = {float}")]
fn y_equal(world: &mut TupleWorld, tuple_name: String, y: f32) {
    assert_eq!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .1,
        y
    );
}

#[then(expr = "{word}.z = {float}")]
fn z_equal(world: &mut TupleWorld, tuple_name: String, z: f32) {
    assert_eq!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .2,
        z
    );
}

#[then(expr = "{word}.w = {float}")]
fn w_equal(world: &mut TupleWorld, tuple_name: String, w: f32) {
    assert_eq!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .3,
        w
    );
}

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut TupleWorld, tuple_name: String) {
    assert!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .is_point()
    );
}

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut TupleWorld, tuple_name: String) {
    assert!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .is_vector()
    );
}

#[then(expr = "{word} is not a point")]
fn is_not_a_point(world: &mut TupleWorld, tuple_name: String) {
    assert!(
        !world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .is_point()
    );
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut TupleWorld, tuple_name: String) {
    assert!(
        !world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str())
            .is_vector()
    );
}

#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn equals_tuple(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(
        *world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str()),
        ExtendedTuple::new(x, y, z, w)
    );
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn tuple_equals_tuple(world: &mut TupleWorld, tuple1: String, tuple2: String) {
    assert!(
        world
            .tuples
            .get(&tuple1)
            .expect(format!("{tuple1} does not exist").as_str())
            == world
                .tuples
                .get(&tuple2)
                .expect(format!("{tuple2} does not exist").as_str()),
    );
}

#[then(expr = "{word} != {word}")]
fn tuple_does_not_equal_tuple(world: &mut TupleWorld, tuple1: String, tuple2: String) {
    assert!(
        !(world
            .tuples
            .get(&tuple1)
            .expect(format!("{tuple1} does not exist").as_str())
            == world
                .tuples
                .get(&tuple2)
                .expect(format!("{tuple2} does not exist").as_str()))
    );
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
fn tuple_added_to_tuple_equals_tuple(
    world: &mut TupleWorld,
    tuple1: String,
    tuple2: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = ExtendedTuple::new(x, y, z, w);

    let tuple1 = world
        .tuples
        .get(&tuple1)
        .expect(format!("{tuple1} does not exist").as_str());

    let tuple2 = world
        .tuples
        .get(&tuple2)
        .expect(format!("{tuple2} does not exist").as_str());

    let result = tuple1 + tuple2;
    assert_eq!(result, expected);
}

#[then(expr = "{word} - {word} = vector\\({float}, {float}, {float})")]
fn point_subtracted_to_point_equals_vector(
    world: &mut TupleWorld,
    point1: String,
    point2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = ExtendedTuple::new_vector(x, y, z);

    let tuple1 = world
        .tuples
        .get(&point1)
        .expect(format!("{point1} does not exist").as_str());

    let tuple2 = world
        .tuples
        .get(&point2)
        .expect(format!("{point2} does not exist").as_str());

    let result = tuple1 - tuple2;
    assert_eq!(result, expected);
}

#[then(expr = "{word} - {word} = point\\({float}, {float}, {float})")]
fn point_subtracted_to_vector_equals_point(
    world: &mut TupleWorld,
    point: String,
    vector: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = ExtendedTuple::new_point(x, y, z);

    let tuple1 = world
        .tuples
        .get(&point)
        .expect(format!("{point} does not exist").as_str());

    let tuple2 = world
        .tuples
        .get(&vector)
        .expect(format!("{vector} does not exist").as_str());

    let result = tuple1 - tuple2;
    assert_eq!(result, expected);
}

#[then(expr = "-{word} = tuple\\({float}, {float}, {float}, {float})")]
fn negated_tuple_equals_tuple(
    world: &mut TupleWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = ExtendedTuple::new(x, y, z, w);

    let tuple = world
        .tuples
        .get(&tuple)
        .expect(format!("{tuple} does not exist").as_str());

    let result = -tuple;
    assert_eq!(result, expected);
}
