use cucumber::{then, when};
use the_ray_tracer_challenge::{Intersection, intersection::Object};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[when(expr = "{word} ← intersection\\({float}, {word})")]
fn intersection_is(world: &mut RayTracerWorld, intersection_name: String, t: f32, sphere: String) {
    let sphere = world.get_sphere(&sphere).clone();
    let intersection = Intersection::new(t, Object::Sphere(sphere));
    world.add_intersection(intersection_name, intersection);
}

#[then(expr = "{word}.t = {float}")]
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
