use cucumber::{then, when};
use the_ray_tracer_challenge::{Computations, Point, Vector};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[when(expr = "{word} ← prepare_computations\\({word}, {word})")]
fn computations_is_prepare_computations_result(
    world: &mut RayTracerWorld,
    computations_name: String,
    intersection: String,
    ray: String,
) {
    let intersection = world.get_intersection(&intersection).clone().unwrap();
    let ray = world.get_ray(&ray).clone();
    let computations = Computations::from_intersection_and_ray(&intersection, &ray);

    world.add_computations(computations_name, computations);
}

#[then(expr = "{word}.normalv = vector\\({int}, {int}, {int})")]
fn computations_normalv_equals_vector(
    world: &mut RayTracerWorld,
    computations: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let computations = world.get_computations(&computations);
    let expected = Vector::new_vector(x, y, z);

    assert_eq!(computations.normal_vector, expected);
}

#[then(expr = "{word}.eyev = vector\\({int}, {int}, {int})")]
fn computations_eyev_equals_vector(
    world: &mut RayTracerWorld,
    computations: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let computations = world.get_computations(&computations);
    let expected = Vector::new_vector(x, y, z);

    assert_eq!(computations.eye_vector, expected);
}

#[then(expr = "{word}.point = point\\({int}, {int}, {int})")]
fn computations_point_equals_point(
    world: &mut RayTracerWorld,
    computations: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let computations = world.get_computations(&computations);
    let expected = Point::new_point(x, y, z);

    assert_eq!(computations.point, expected);
}
#[then(expr = "{word}.t = {word}.t")]
fn computations_t_equals_intersection_t(
    world: &mut RayTracerWorld,
    computations: String,
    intersection: String,
) {
    let computations = world.get_computations(&computations);
    let intersection = world.get_intersection(&intersection).as_ref().unwrap();

    assert_eq!(computations.t, intersection.t());
}

#[then(expr = "{word}.object = {word}.object")]
fn computation_object_equals_intersection_object(
    world: &mut RayTracerWorld,
    computation: String,
    intersection: String,
) {
    let computations = world.get_computations(&computation);
    let intersection = world.get_intersection(&intersection).to_owned().unwrap();
    assert_eq!(computations.object, intersection.object());
}
