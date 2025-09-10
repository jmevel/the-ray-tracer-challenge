use cucumber::{given, then, World};
use the_ray_tracer_challenge::TupleExt;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuple: (f32, f32, f32, f32),
}

#[given(expr = "a ← tuple\\({float}, {float}, {float}, {float})")]
fn a_tuple(world: &mut TupleWorld, x: f32, y: f32, z: f32, w: f32) {
    world.tuple = (x, y, z, w);
}

#[then(expr = "a.x = {float}")]
fn x_equal(world: &mut TupleWorld, x: f32) {
    assert_eq!(world.tuple.0, x);
}

#[then(expr = "a.y = {float}")]
fn y_equal(world: &mut TupleWorld, y: f32) {
    assert_eq!(world.tuple.1, y);
}

#[then(expr = "a.z = {float}")]
fn z_equal(world: &mut TupleWorld, z: f32) {
    assert_eq!(world.tuple.2, z);
}

#[then(expr = "a.w = {float}")]
fn w_equal(world: &mut TupleWorld, w: f32) {
    assert_eq!(world.tuple.3, w);
}

#[then(expr = "a is a point")]
fn a_is_a_point(world: &mut TupleWorld) {
    assert!(world.tuple.is_point());
}

#[then(expr = "a is a vector")]
fn a_is_a_vector(world: &mut TupleWorld) {
    assert!(world.tuple.is_vector());
}

#[then(expr = "a is not a point")]
fn a_is_not_a_point(world: &mut TupleWorld) {
    assert!(!world.tuple.is_point());
}

#[then(expr = "a is not a vector")]
fn a_is_not_a_vector(world: &mut TupleWorld) {
    assert!(!world.tuple.is_vector());
}

