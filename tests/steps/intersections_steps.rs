use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Intersection, intersection::Object};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← intersection\\({float}, {word})")]
#[when(expr = "{word} ← intersection\\({float}, {word})")]
fn intersection_is(world: &mut RayTracerWorld, intersection_name: String, t: f32, sphere: String) {
    let sphere = world.get_sphere(&sphere).clone();
    let intersection = Intersection::new(t, Object::Sphere(sphere));
    world.add_intersection(intersection_name, intersection);
}

#[when(expr = "{word} ← intersections\\({word}, {word})")]
fn intersection_collections_is(
    world: &mut RayTracerWorld,
    intersection_collection_name: String,
    intersection1: String,
    intersection2: String,
) {
    let intersections_collection = vec![intersection1, intersection2];
    world.add_intersections_collection(intersection_collection_name, intersections_collection);
}

// #[then(expr = "{word}.t = {float}")]
#[then(
    regex = r#"^([a-zA-Z0-9_]+)\.t = ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))$"#
)]
fn t_of_intersection_equals(world: &mut RayTracerWorld, intersection_name: String, t: f32) {
    let intersection = world.get_intersection(&intersection_name);
    assert_eq!(intersection.t(), t);
}

#[then(expr = "{word}.object = {word}")]
fn object_of_intersection_equals_sphere(
    world: &mut RayTracerWorld,
    intersection_name: String,
    sphere: String,
) {
    let intersection = world.get_intersection(&intersection_name);
    let sphere = world.get_sphere(&sphere).clone();
    assert_eq!(intersection.object(), &Object::Sphere(sphere));
}

#[then(expr = "{word}[{int}].t = {float}")]
fn t_of_intersection_at_index_equals(
    world: &mut RayTracerWorld,
    intersections_collection_name: String,
    index: usize,
    t: f32,
) {
    let intersection = world.get_intersections_collection(&intersections_collection_name)[index];
    assert_eq!(intersection.t(), t);
}
