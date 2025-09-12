use cucumber::{given, then, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::tuples;
use the_ray_tracer_challenge::TupleExt;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuples: HashMap<String, (f32, f32, f32, f32)>, //tuple: (f32, f32, f32, f32),
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuples.insert(tuple_name, (x, y, z, w));
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn point(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(tuple_name, tuples::point(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn vector(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(tuple_name, tuples::vector(x, y, z));
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

#[then(expr = "{word} = tuple\\({float}, {float}, {float}, {float})")]
fn equals_tuple(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(
        world
            .tuples
            .get(&tuple_name)
            .expect(format!("{tuple_name} does not exist").as_str()),
        &(x, y, z, w)
    );
}

#[then(expr = "{word} equals {word}")]
fn tuple_equals_tuple(world: &mut TupleWorld, tuple_name1: String, tuple_name2: String) {
    assert!(
        world
            .tuples
            .get(&tuple_name1)
            .expect(format!("{tuple_name1} does not exist").as_str())
            .equals(
                world
                    .tuples
                    .get(&tuple_name2)
                    .expect(format!("{tuple_name2} does not exist").as_str()),
            )
    );
}

#[then(expr = "{word} does not equal {word}")]
fn tuple_does_not_equal_tuple(world: &mut TupleWorld, tuple_name1: String, tuple_name2: String) {
    assert!(
        !world
            .tuples
            .get(&tuple_name1)
            .expect(format!("{tuple_name1} does not exist").as_str())
            .equals(
                world
                    .tuples
                    .get(&tuple_name2)
                    .expect(format!("{tuple_name2} does not exist").as_str()),
            )
    );
}
