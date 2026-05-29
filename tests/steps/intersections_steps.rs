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
    intersection1_name: String,
    intersection2_name: String,
) {
    let intersection1 = world.get_intersection(&intersection1_name);
    let intersection2 = world.get_intersection(&intersection2_name);
    let intersections_collection = vec![intersection1.clone(), intersection2.clone()];
    world
        .add_intersections_collection(intersection_collection_name, Some(intersections_collection));
}

#[then(
    regex = r#"^([a-zA-Z0-9_]+)\.t = ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))$"#
)]
fn t_of_intersection_equals(world: &mut RayTracerWorld, intersection_name: String, t: f32) {
    let intersection = world.get_intersection(&intersection_name);
    assert_eq!(intersection.t(), t);
}

#[then(expr = "{word}[{int}].t = {float}")]
fn t_of_intersection_at_index_equals(
    world: &mut RayTracerWorld,
    intersections_collection_name: String,
    index: usize,
    t: f32,
) {
    let intersections = world
        .get_intersections_collection(&intersections_collection_name)
        .clone()
        .unwrap();
    let actual = &intersections[index].t();
    assert_eq!(actual, &t);
}
